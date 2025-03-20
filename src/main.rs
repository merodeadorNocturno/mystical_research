use crate::utils::env_utils::*;
use actix_cors::Cors;
use actix_web::{middleware, web::Data, App, HttpServer};
use controllers::{
    about_controller::about_html, blog_api_controller::blog_api_routes,
    blog_controller::blog_home_html, contact_controller::contact_html,
    index_controller::index_html, resources_controller::resources_html,
    static_controllers::static_controllers, topics_controller::topics_html,
};
use db::config_db::Database;
use env_logger::{Builder, WriteStyle};
use log::{info, warn};
mod config;
mod controllers;
mod db;
mod models;
mod utils;

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

    let my_db = Database::init().await.expect("CANT_CONNECT_TO_DB");
    let db_data = Data::new(my_db);

    let server_address_conf: String = format!("{server_address}:{server_port}");

    HttpServer::new(move || {
        let cors = Cors::permissive().max_age(MAX_AGE);
        App::new()
            .wrap(cors)
            .wrap(middleware::NormalizePath::trim())
            .app_data(db_data.clone())
            .configure(blog_api_routes)
            .configure(index_html)
            .configure(about_html)
            .configure(resources_html)
            .configure(topics_html)
            .configure(contact_html)
            .configure(blog_home_html)
            .configure(static_controllers)
    })
    .bind(server_address_conf)
    .expect("FAILED TO BIND TO PORT")
    .run()
    .await
}
