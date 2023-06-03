# Settings

`setting-env` is a rust package to load settings from environment variables.

## Usage

```rust
use settings_env::Settings;

#[derive(Settings)]
struct AppSettings {
    pub host: String,
    pub port: u16,
}


fn main() {
    let settings = AppSettings::load_from_env();
}
```
