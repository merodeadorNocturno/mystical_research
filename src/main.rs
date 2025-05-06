use crate::utils::{
    env_utils::*,
    general_utils::{create_robots_txt_template, create_sitemap_xml_template, get_template_path},
};
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{middleware, web::Data, App, HttpServer};
use controllers::{
    about_controller::about_controller, blog_api_controller::blog_api_controller,
    blog_controller::blog_html_controller, contact_controller::contact_controller,
    index_controller::index_controller, mailing_controller::mailing_list_controller,
    resources_controller::resources_controller, static_controllers::static_controllers,
    topics_controller::topics_controller,
};
use db::config_db::Database;
use env_logger::{Builder, WriteStyle};
use log::{error, info, warn};
use std::{fs as std_fs, io, path::PathBuf};

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
        Ok(cwd) => info!("Successfully retrieved current directory: {:?}", cwd),
        Err(err) => warn!("Error getting current directory: {}", err),
    }

    let PageConfiguration {
        server_address,
        server_port,
        public_base_url,
        ..
    } = set_env_urls();

    let template_path = get_template_path();

    let static_asset_path = PathBuf::from("static");

    if let Err(e) = std_fs::create_dir_all(&template_path) {
        error!(
            "Failed to create template directory {:?}: {}",
            template_path, e
        );
        return Err(e);
    }

    if let Err(e) = std_fs::create_dir_all(&static_asset_path) {
        error!(
            "Failed to create static asset directory {:?}: {}",
            static_asset_path, e
        );
        return Err(e);
    }

    if let Err(e) = create_robots_txt_template(&template_path, &public_base_url) {
        error!("Failed to create robots.txt: {}", e);

        return Err(io::Error::new(io::ErrorKind::Other, e));
    }

    if let Err(e) = create_sitemap_xml_template(&template_path, &public_base_url) {
        error!("Failed to create sitemap.xml: {}", e);

        return Err(io::Error::new(io::ErrorKind::Other, e));
    }

    let my_db = Database::init().await.expect("CANT_CONNECT_TO_DB");
    let db_data = Data::new(my_db);

    info!(
        "DB up and running:: {} :: {}",
        db_data.db_name, db_data.name_space
    );

    let server_address_conf: String = format!("{server_address}:{server_port}");

    HttpServer::new(move || {
        let cors = Cors::permissive().max_age(MAX_AGE);
        App::new()
            .wrap(cors)
            .wrap(middleware::NormalizePath::trim())
            .app_data(db_data.clone())
            .configure(blog_api_controller)
            .configure(index_controller)
            .configure(about_controller)
            .configure(resources_controller)
            .configure(topics_controller)
            .configure(contact_controller)
            .configure(mailing_list_controller)
            .configure(blog_html_controller)
            .configure(static_controllers)
            .service(fs::Files::new("/static", static_asset_path.clone()))
    })
    .bind(server_address_conf)
    .expect("FAILED TO BIND TO PORT")
    .run()
    .await
}
