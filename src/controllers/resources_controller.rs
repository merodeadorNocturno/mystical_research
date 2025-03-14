use crate::utils::fs_utils::{read_hbs_template, register_templates};
use actix_web::{
    web::{get, ServiceConfig},
    HttpResponse,
};
use handlebars::{Handlebars, RenderError};
use log::error;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
struct TitleError {
    pub error: String,
}

impl TitleError {
    pub fn new(error: String) -> TitleError {
        TitleError { error }
    }
}

async fn load_resources() -> Result<String, RenderError> {
    let mut handlebars = Handlebars::new();
    let this_path = Path::new("./static/templates");

    register_templates(this_path, &mut handlebars);
    let resources_hbs = "resources/resources";

    let section_template = match read_hbs_template(&resources_hbs) {
        Ok(contents) => contents,
        Err(err) => {
            error!(
                "Failed to render contents for resources page: {}",
                err.to_string()
            );
            TitleError::new(err.to_string()).error
        }
    };

    let section_template = handlebars.render_template(&section_template, &json!({}))?;
    Ok(section_template)
}

pub fn resources_html(cfg: &mut ServiceConfig) {
    cfg.route(
        "/resources",
        get().to(|| async move {
            let resources_template = load_resources().await;
            match resources_template {
                Ok(template) => HttpResponse::Ok()
                    .content_type("text/html")
                    .body(template),
                Err(err) => HttpResponse::InternalServerError()
                    .content_type("text/html")
                    .body(format!(
                        "<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load resources page: {}</span>",
                        err.to_string()
                    )),
            }
        }),
    );
}
