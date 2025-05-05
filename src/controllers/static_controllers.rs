use crate::utils::env_utils::{get_cwd, set_env_urls, PageConfiguration};
use actix_files as fs;
use actix_web::{
    get,
    http::header::{ContentDisposition, DispositionType},
    web::ServiceConfig,
    Error, HttpRequest, HttpResponse, Responder,
};
use log::{error, info, warn};
use std::path::PathBuf;
use tokio::fs::read_to_string;

#[get("/{filename:.*}")]
async fn scripts_static(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    match get_cwd() {
        Ok(_) => info!("Successfully retrieved current working directory"),
        Err(e) => warn!("Error retrieving current working directory: {}", e),
    }

    let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();

    let file = fs::NamedFile::open_async(path).await?;

    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        }))
}

// Handler to serve the robots.txt file
#[get("/robots.txt")]
async fn serve_robots_txt(_req: HttpRequest) -> impl Responder {
    let PageConfiguration { template_path, .. } = set_env_urls();
    let mut robots_file_path = PathBuf::from(template_path);
    robots_file_path.push("robots.txt");

    match read_to_string(&robots_file_path).await {
        Ok(content) => HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .body(content),
        Err(e) => {
            error!(
                "Failed to read robots.txt file at {:?}: {}",
                robots_file_path, e
            );
            // Return a generic 404 or an internal server error
            HttpResponse::NotFound()
                .content_type("text/plain")
                .body("Robots.txt not found.")
            // Or HttpResponse::InternalServerError().finish() if it's unexpected
        }
    }
}

pub fn static_controllers(cfg: &mut ServiceConfig) {
    cfg.service(scripts_static);
    cfg.service(serve_robots_txt);
}
