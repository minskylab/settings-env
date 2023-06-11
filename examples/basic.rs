use std::env;

use settings_env::Settings;

struct ServiceSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Settings)]
// #[settings(prefix = "app")]
struct AppSettings {
    pub debug: bool,
    pub service: ServiceSettings,
}

fn main() {
    env::set_var("host", "0.0.0.0");
    env::set_var("port", "8080");

    let settings = AppSettings::load_from_env();

    assert_eq!(
        Some(settings.debug),
        env::var("APP_DEBUG").unwrap().parse::<bool>().ok()
    );
    assert_eq!(
        Some(settings.service.host),
        env::var("APP_SERVICE_HOST").ok()
    );
    assert_eq!(
        Some(settings.service.port),
        env::var("APP_SERVICE_PORT").unwrap().parse::<u16>().ok()
    );
}
