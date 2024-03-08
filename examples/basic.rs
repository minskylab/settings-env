use std::env;

use serde::Deserialize;
use serde_json::Deserializer;
use settings_env::{deserializer::SettingsDeserializer, Settings};

#[derive(Deserialize, Debug)]
struct ServiceSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Debug)]
// #[settings(prefix = "app")]
struct AppSettings {
    pub debug: bool,
    pub service: ServiceSettings,
}

fn main() {
    env::set_var("host", "0.0.0.0");
    env::set_var("port", "8080");

    // let settings = AppSettings::load().unwrap();

    // assert_eq!(
    //     Some(settings.debug),
    //     env::var("APP_DEBUG").unwrap().parse::<bool>().ok()
    // );
    // assert_eq!(
    //     Some(settings.service.host),
    //     env::var("APP_SERVICE_HOST").ok()
    // );
    // assert_eq!(
    //     Some(settings.service.port),
    //     env::var("APP_SERVICE_PORT").unwrap().parse::<u16>().ok()
    // );

    // let mut des =

    // let mut de = Deserializer::new(env::vars());

    let res = AppSettings::deserialize(SettingsDeserializer {
        prefix: "APP".to_string(),
    })
    .unwrap();

    // // des.prefix = "APP_SERVICE".to_string();

    // let res = ServiceSettings::deserialize(SettingsDeserializer {
    //     prefix: "APP_SERVICE".to_string(),
    // })
    // .unwrap();

    // println!("{:?}", res);
}
