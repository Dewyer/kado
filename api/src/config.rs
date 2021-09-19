use std::collections::HashMap;
use std::env;

use rocket::config::{Config, Environment, Value, Limits};
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
    pub admin_key: String,
    pub google_client_id: String,
    pub aws_access_key: String,
    pub aws_secret_key: String,
    pub aws_bucket: String,
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
                google_client_id: env::var("GOOGLE_CLIENT_ID").expect("No google client id in environment!"),
                secret: secret.into_bytes(),
                admin_key: env::var("ADMIN_KEY").expect("No admin key in environment!"),
                aws_access_key: env::var("A_AWS_ACCESS_KEY").expect("No aws key in environment!"),
                aws_secret_key: env::var("A_SECRET_KEY").expect("No aws secret key in environment!"),
                aws_bucket: env::var("A_AWS_BUCKET").expect("No aws bucket in environment!")
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

    let mut builder =     Config::build(environment)
        .environment(environment);

    if env::var("PROD").unwrap_or("false".to_string()) != "true" {
        builder = builder.address("127.0.0.1");
    }

    builder
        .port(port)
        .extra("databases", databases)
        .limits(Limits::new()
                    .limit("forms", 3 * 1024 * 1024)
                    .limit("json", 3 * 1024 * 1024))
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
