use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use controllers::static_controllers::static_controllers;
use env_logger::{Builder, WriteStyle};
use log::{info, warn};
mod constants;
mod controllers;
mod models;
mod utils;
use crate::utils::env::*;

const MAX_AGE: usize = 3600;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = Builder::new();

    builder
        .filter_level(log::LevelFilter::Info)
        .write_style(WriteStyle::Always)
        .init();

    match get_cwd() {
        Ok(_) => info!("Successfully retrieved current directory"),
        Err(err) => warn!("Error getting current directory: {}", err),
    }

    let PageConfiguration {
        server_address,
        server_port,
        ..
    } = set_env_urls();

    let server_address_conf: String = format!("{server_address}:{server_port}");

    HttpServer::new(move || {
        let cors = Cors::permissive().max_age(MAX_AGE);
        App::new()
            .wrap(cors)
            .wrap(middleware::NormalizePath::trim())
            .configure(static_controllers)
    })
    .bind(server_address_conf)
    .expect("FAILED TO BIND TO PORT")
    .run()
    .await
}
