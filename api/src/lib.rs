#![feature(proc_macro_hygiene, decl_macro)]
#![feature(option_result_contains)]
#![recursion_limit = "256"]

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate thiserror;

use dotenv::dotenv;
use rocket_contrib::json::JsonValue;
use rocket_cors;
use rocket_cors::{AllowedOrigins, Cors, CorsOptions};
use rocket_okapi::routes_with_openapi;
use rocket_okapi::swagger_ui::make_swagger_ui;

use crate::config::get_swagger_config;

mod guards;
mod config;
mod db;
mod errors;
mod models;
mod routes;
mod schema;
mod services;

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "errors",
        "reason": "Resource was not found."
    })
}

fn cors_fairing() -> Cors {
    let allowed_origins = AllowedOrigins::All;

    Cors::from_options(&CorsOptions {
        allowed_origins,
        ..CorsOptions::default()
    })
    .expect("Cors fairing cannot be created")
}

pub fn rocket() -> rocket::Rocket {
    dotenv().ok();

    rocket::custom(config::from_env())
        .mount(
            "/api",
            routes_with_openapi![
                routes::healthcheck_routes::healthcheck_route,
                routes::auth_routes::check_user,
                routes::auth_routes::refresh_token,
                routes::auth_routes::register_user,
                routes::auth_routes::login_user,
            ],
        )
        .mount("/swagger", make_swagger_ui(&get_swagger_config()))
        .attach(db::ConnPool::fairing())
        .attach(cors_fairing())
        .attach(config::AppConfig::manage())
        .register(catchers![not_found])
}
