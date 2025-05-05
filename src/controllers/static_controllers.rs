use crate::utils::general_utils::get_template_path;
use actix_web::{get, web::ServiceConfig, HttpRequest, HttpResponse, Responder};
use log::{error, info};
use tokio::fs::read_to_string;

// #[get("/{filename:.*}")]
// async fn scripts_static(req: HttpRequest) -> Result<fs::NamedFile, Error> {
//     match get_cwd() {
//         Ok(_) => info!("Successfully retrieved current working directory"),
//         Err(e) => warn!("Error retrieving current working directory: {}", e),
//     }

//     let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();

//     let file = fs::NamedFile::open_async(path).await?;

//     Ok(file
//         .use_last_modified(true)
//         .set_content_disposition(ContentDisposition {
//             disposition: DispositionType::Inline,
//             parameters: vec![],
//         }))
// }

// Handler to serve the robots.txt file
#[get("/robots.txt")]
async fn serve_robots_txt(_req: HttpRequest) -> impl Responder {
    // Determine the path *once* at startup if possible, or use a helper
    let mut robots_file_path = get_template_path(); // Use helper
    robots_file_path.push("robots.txt");
    info!(
        "Attempting to serve robots.txt from: {:?}",
        robots_file_path
    ); // Added logging

    match read_to_string(&robots_file_path).await {
        Ok(content) => HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .body(content),
        Err(e) => {
            error!(
                "Failed to read robots.txt file at {:?}: {}",
                robots_file_path, e
            );
            HttpResponse::NotFound()
                .content_type("text/plain")
                .body("Robots.txt not found.")
        }
    }
}

// Handler to serve the sitemap.xml file
#[get("/sitemap.xml")]
async fn serve_sitemap_xml(_req: HttpRequest) -> impl Responder {
    let mut sitemap_file_path = get_template_path(); // Use helper
    sitemap_file_path.push("sitemap.xml");
    info!(
        "Attempting to serve sitemap.xml from: {:?}",
        sitemap_file_path
    ); // Added logging

    match read_to_string(&sitemap_file_path).await {
        Ok(content) => HttpResponse::Ok()
            .content_type("application/xml; charset=utf-8") // Correct content type
            .body(content),
        Err(e) => {
            error!(
                "Failed to read sitemap.xml file at {:?}: {}",
                sitemap_file_path, e
            );
            HttpResponse::NotFound()
                .content_type("text/plain")
                .body("Sitemap.xml not found.")
        }
    }
}

pub fn static_controllers(cfg: &mut ServiceConfig) {
    cfg.service(serve_robots_txt);
    cfg.service(serve_sitemap_xml);
}
