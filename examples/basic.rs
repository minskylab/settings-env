// use crate::AnswerFn;
use std::env;

use settings_env::Settings;

#[derive(Settings)]
struct AppSettings {
    pub host: String,
    pub port: u16,
}

fn main() {
    // SettingsEnvLoader
    env::set_var("host", "0.0.0.0");
    env::set_var("port", "8080");

    let settings = AppSettings::load_from_env();

    // let settings = Settings::parse();
    // assert_eq!(settings.host,
}
