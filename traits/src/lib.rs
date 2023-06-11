use std::collections::BTreeMap;

pub trait FromEnv {
    fn from_env(namespace: String, defaults: BTreeMap<String, String>) -> Self;
}

pub trait SettingsEnvLoader {
    fn load_from_env() -> String;
}
