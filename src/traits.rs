use std::collections::BTreeMap;

pub trait FromEnv {
    fn from_env(defaults: BTreeMap<String, String>) -> Self;
}

pub trait SettingsEnvLoader {
    fn load() -> String;
}
