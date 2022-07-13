/*!
# CDDIO

Bot Discord officiel du serveur Coin des Developpeurs ([Rejoignez nous !](https://discord.gg/m9EZNKVaPz))

Crée par la communauté pour la communauté.

Ce bot est développé en [**Rust**](https://www.rust-lang.org/) et repose sur la crate [`serenity`], [`cddio_core`] et [`cddio_macros`].

## Fonctionnalités

* [*Autobahn*, l'anti spam](src/component_system/components/autobahn/)
* [Aide du bot](src/component_system/components/help/)
* [Commandes diverses](src/component_system/components/misc/)
* [Commandes de modération](src/component_system/components/modo/)
* [Déclaration des slash commands](src/component_system/components/slash/)
* [Gestion de ticket du serveur](src/component_system/components/tickets/)

## Licence

Ce projet est licencié sous **GPLv3**. 
Je vous invite à aller [sur cette page](https://choosealicense.com/licenses/gpl-3.0/) pour plus de renseignement.
*/
#![allow(unused_macros, dead_code)]
mod bot;
mod components;
mod config;
mod log;

/// Trait à implémenter pour logger les erreurs dans la console.
trait ResultLog {
    type OkType;
    /// Si une erreur se produit, panic et log le message en entrée et l'erreur.
    /// Sinon, renvoie la valeur.
    fn expect_log(self, msg: &str) -> Self::OkType;
}
impl<T, S: AsRef<str>> ResultLog for Result<T, S> {
    type OkType=T;
    fn expect_log(self, msg: &str) -> T {
        match self {
            Ok(v) => v,
            Err(e) if msg.is_empty() => panic!("{}", e.as_ref()),
            Err(e) => panic!("{}: {}", msg, e.as_ref()),
        } 
    }
}

#[tokio::main]
async fn main() {
    if let Err(e) =  log::init() {
        panic!("Unable to set logger: {}", e);
    }
    let config = config::Config::load("./config.ron").expect_log("Could not load the configuration file");
    let mut bot = bot::Bot::new(&config).await
        .or_else(|e|Err(e.to_string()))
        .expect_log("");
    bot
        .start().await
        .or_else(|e| Err(e.to_string()))
        .expect_log("Could not start the bot");
}
