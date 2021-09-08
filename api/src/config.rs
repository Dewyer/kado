use std::collections::HashMap;
use std::env;

use rocket::config::{Config, Environment, Value};
use rocket::fairing::AdHoc;
use rocket_okapi::swagger_ui::SwaggerUIConfig;

/// Debug only secret for JWT encoding & decoding.
const SECRET: &'static str = "KKEZxdXd";

/// js toISOString() in test suit can't handle chrono's default precision
// pub const DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";

// pub const TOKEN_PREFIX: &'static str = "Token ";
#[derive(Clone)]
pub struct AppConfig {
    pub secret: Vec<u8>,
    pub google_client_id: String,
}

impl AppConfig {
    pub fn manage() -> AdHoc {
        AdHoc::on_attach("Manage config", |rocket| {
            // Rocket doesn't expose it's own secret_key, so we use our own here.
            let secret = env::var("SECRET_KEY").unwrap_or_else(|err| {
                if cfg!(debug_assertions) {
                    SECRET.to_string()
                } else {
                    panic!("No SECRET_KEY environment variable found: {:?}", err)
                }
            });

            Ok(rocket.manage(AppConfig {
                google_client_id: env::var("GOOGLE_CLIENT_ID").expect("No google client id defined!"),
                secret: secret.into_bytes(),
            }))
        })
    }
}

/// Create rocket config from environment variables
pub fn from_env() -> Config {
    let environment = Environment::active().expect("No environment found");

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT environment variable should parse to an integer");

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    let database_url =
        env::var("DATABASE_URL").expect("No DATABASE_URL environment variable found");
    database_config.insert("url", Value::from(database_url));
    databases.insert("diesel_postgres_pool", Value::from(database_config));

    Config::build(environment)
        .environment(environment)
        .address("127.0.0.1")
        .port(port)
        .extra("databases", databases)
        .finalize()
        .unwrap()
}

pub fn get_swagger_config() -> SwaggerUIConfig {
    use rocket_okapi::swagger_ui::UrlObject;

    SwaggerUIConfig {
        url: Option::from("/api/openapi.json".to_string()),
        urls: Option::from(vec![UrlObject::new("Voting Api", "/api/openapi.json")]),
        ..Default::default()
    }
}
