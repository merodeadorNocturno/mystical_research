use crate::utils::{
    env_utils::*,
    general_utils::{create_robots_txt_template, create_sitemap_xml_template, get_template_path},
};
use actix_cors::Cors;
use actix_csrf::CsrfMiddleware;
use actix_files as fs;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_session::{SessionMiddleware, config::PersistentSession, storage::CookieSessionStore};
use actix_web::cookie::Key;
use actix_web::http::Method; // Added for CsrfMiddleware configuration
use actix_web::{App, HttpServer, middleware, web::Data};
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
use rand::rngs::StdRng;
use std::{fs as std_fs, io, path::PathBuf};

mod config;
mod controllers;
mod db;
mod models;
mod utils;

const MAX_AGE: usize = 3_600;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Install the default crypto provider (using ring)
    let _ = rustls::crypto::ring::default_provider().install_default();
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

    // IMPORTANT: In a real application, load this key from a secure, persistent
    // configuration or environment variable. Generating it on startup means all
    // sessions are invalidated on restart.
    let secret_key = Key::generate();

    info!(
        "DB up and running:: {} :: {}",
        db_data.db_name, db_data.name_space
    );

    let server_address_conf: String = format!("{server_address}:{server_port}");

    let gov_conf = get_gvt_conf();
    let governor_cof = GovernorConfigBuilder::default()
        .requests_per_second(gov_conf.requests_per_second as u64)
        .burst_size(gov_conf.burst_size as u32)
        .finish()
        .unwrap();

    HttpServer::new(move || {
        let cors = Cors::permissive().max_age(MAX_AGE);
        let session_mw =
            SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                .cookie_secure(true) // Set to true if using HTTPS
                .cookie_same_site(actix_web::cookie::SameSite::Lax)
                .session_lifecycle(
                    PersistentSession::default()
                        .session_ttl(actix_web::cookie::time::Duration::days(1)),
                )
                .build();

        App::new()
            .wrap(
                CsrfMiddleware::<StdRng>::new()
                    .set_cookie(Method::GET, "/contact.html")
                    .set_cookie(Method::GET, "/htmx/contact"),
            )
            .wrap(session_mw)
            .wrap(cors)
            .wrap(middleware::NormalizePath::trim())
            .wrap(Governor::new(&governor_cof))
            .wrap(middleware::DefaultHeaders::new().add(("X-Frame-Options", "DENY")))
            .wrap(middleware::Logger::new("%{r}a %r %s %b %T"))
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
