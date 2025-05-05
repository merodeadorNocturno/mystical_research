use log::info;
use serde::{Deserialize, Serialize};
use std::{env, io};

use crate::config::connection::set_environment_variable;

pub fn get_cwd() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    info!("Current working directory: {}", current_dir.display());

    Ok(())
}

fn get_server_conf() -> String {
    let server_address = set_environment_variable("SERVER_ADDRESS", "0.0.0.0");
    let server_port = set_environment_variable("SERVER_PORT", "8081");
    let server_protocol = set_environment_variable("SERVER_PROTOCOL", "http");

    format!("{server_protocol}://{server_address}:{server_port}/")
}

fn get_backend_url() -> String {
    let backend_address = set_environment_variable("BACKEND_ADDRESS", "0.0.0.0");
    let mut backend_port = set_environment_variable("BACKEND_PORT", "8080");
    let backend_protocol = set_environment_variable("BACKEND_PROTOCOL", "http");

    if backend_port != *"80" && backend_port != *"" {
        backend_port = format!(":{}", &backend_port);
    } else {
        backend_port = "".to_string();
    }
    format!("{backend_protocol}://{backend_address}{backend_port}/")
}

// pub fn get_log_level() -> LevelFilter {
//     let log_level = set_environment_variable("RUST_LOG", "debug");

//     let level_filter = match log_level.as_str() {
//         "off" => LevelFilter::Off,
//         "error" => LevelFilter::Error,
//         "warn" => LevelFilter::Warn,
//         "info" => LevelFilter::Info,
//         "debug" => LevelFilter::Debug,
//         "trace" => LevelFilter::Trace,
//         _ => LevelFilter::Debug,
//     };

//     level_filter
// }

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PageConfiguration {
    pub server_conf: String,
    pub backend_url: String,
    pub server_address: String,
    pub server_port: String,
    pub server_protocol: String,
    pub title: String,
    pub ai_google_api_key: String,
    pub ai_request_url: String,
    pub google_model: String,
    pub template_path: String,
    pub public_base_url: String,
}

pub fn set_env_urls() -> PageConfiguration {
    PageConfiguration {
        server_conf: get_server_conf(),
        backend_url: get_backend_url(),
        server_address: set_environment_variable("SERVER_ADDRESS", "0.0.0.0"),
        server_port: set_environment_variable("SERVER_PORT", "8081"),
        server_protocol: set_environment_variable("SERVER_PROTOCOL", "http"),
        title: set_environment_variable("PAGE_TITLE", "CRM"),
        ai_google_api_key: set_environment_variable("AI_GOOGLE_API_KEY", "some_api_key"),
        ai_request_url: set_environment_variable(
            "AI_REQUEST_URL",
            "https://generativelanguage.googleapis.com/v1beta/models/",
        ),
        google_model: set_environment_variable("GOOGLE_MODEL", "gemini-2.0-flash"),
        template_path: set_environment_variable("TEMPLATE_PATH", "./static/templates"),
        public_base_url: set_environment_variable("PUBLIC_BASE_URL", "https://example.com"),
    }
}

pub fn create_ai_request_string() -> String {
    let PageConfiguration {
        ai_google_api_key,
        ai_request_url,
        google_model,
        ..
    } = set_env_urls();
    format!("{ai_request_url}{google_model}:generateContent?key={ai_google_api_key}")
}
