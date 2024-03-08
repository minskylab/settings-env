use askama::Template;
struct OrganizationSettings {
    domain: String,
    name: String,
}

struct DatabaseSettings {
    host: String,
    port: u16,
    username: String,
    password: String,
}

#[derive(Template)]
#[template(path = "settings.jinja")]
struct AppSettings {
    name: String,
    database: DatabaseSettings,
    organization: OrganizationSettings,
}

fn main() {
    // let settings = AppSettings::render().unwrap();
}
