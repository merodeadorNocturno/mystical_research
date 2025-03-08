use crate::models::index::IndexPage;
use crate::models::mock::{
    mock_header_data, mock_index_body, mock_index_featured_section, mock_index_schema_markup,
};
use crate::utils::{
    env::set_env_urls,
    fs_utils::{read_hbs_template, register_templates},
};
use actix_web::{
    web::{get, ServiceConfig},
    HttpResponse,
};
use handlebars::{Handlebars, RenderError};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
struct Title {
    title: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TitleError {
    pub error: String,
}

impl TitleError {
    pub fn new(error: String) -> TitleError {
        TitleError { error }
    }
}

async fn load_html() -> Result<String, RenderError> {
    let mut handlebars = Handlebars::new();
    let this_path = Path::new("./src/static");

    register_templates(this_path, &mut handlebars);
    let index_hbs = "index";

    let section_template = match read_hbs_template(&index_hbs) {
        Ok(contents) => contents,
        Err(err) => {
            error!(
                "Failed to render contents for edit title:: {}",
                err.to_string()
            );
            TitleError::new(err.to_string()).error
        }
    };

    let schema_markup = mock_index_schema_markup();
    let body = mock_index_body();
    let header = mock_header_data();
    let mut featured = Vec::new();

    featured.push(mock_index_featured_section("article1.png".to_string()));
    featured.push(mock_index_featured_section("article2.png".to_string()));
    featured.push(mock_index_featured_section("article3.png".to_string()));

    let section_template = handlebars.render_template(
        &section_template,
        &json!(&IndexPage {
            body,
            schema_markup,
            featured,
            header,
        }),
    )?;
    Ok(section_template)
}

pub fn index_html(cfg: &mut ServiceConfig) {
    cfg.route(
      "/",
      get().to(|| async move {
        let mr_help_template = load_html().await;
        match mr_help_template {
            Ok(template) => HttpResponse::Ok()
              .content_type("text/html")
              .append_header(("HX-Trigger", "help_table"))
             .body(template),
            Err(err) => HttpResponse::InternalServerError()
              .content_type("text/html")
              .append_header(("HX-Trigger", "help_table"))
              .body(format!("<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load title: {}</span>",
              err.to_string())),
        }
    }));
}
