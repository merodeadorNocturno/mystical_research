use crate::models::ai::{Content, GenerateContentRequest, Part};
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

async fn load_blog_home() -> Result<String, RenderError> {
    let mut handlebars = Handlebars::new();
    let this_path = Path::new("./src/static");

    register_templates(this_path, &mut handlebars);
    let blog_home_hbs = "blog_home";

    let section_template = match read_hbs_template(&blog_home_hbs) {
        Ok(contents) => contents,
        Err(err) => {
            error!(
                "Failed to render contents for blog home page: {}",
                err.to_string()
            );
            TitleError::new(err.to_string()).error
        }
    };

    let section_template = handlebars.render_template(&section_template, &json!({}))?;
    Ok(section_template)
}

async fn load_blog_article() -> Result<String, RenderError> {
    let mut handlebars = Handlebars::new();
    let this_path = Path::new("./src/static");

    register_templates(this_path, &mut handlebars);
    let blog_article_hbs = "blog_article";

    let section_template = match read_hbs_template(&blog_article_hbs) {
        Ok(contents) => contents,
        Err(err) => {
            error!(
                "Failed to render contents for blog article page: {}",
                err.to_string()
            );
            TitleError::new(err.to_string()).error
        }
    };

    let section_template = handlebars.render_template(&section_template, &json!({}))?;
    Ok(section_template)
}

pub fn blog_home_html(cfg: &mut ServiceConfig) {
    cfg.route(
        "/blog",
        get().to(|| async move {
            let blog_home_template = load_blog_home().await;
            match blog_home_template {
                Ok(template) => HttpResponse::Ok()
                    .content_type("text/html")
                    .body(template),
                Err(err) => HttpResponse::InternalServerError()
                    .content_type("text/html")
                    .body(format!(
                        "<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load blog home page: {}</span>",
                        err.to_string()
                    )),
            }
        }),
    );

    cfg.route(
            "/blog/article",
            get().to(|| async move {
                let blog_article_template = load_blog_article().await;
                match blog_article_template {
                    Ok(template) => HttpResponse::Ok()
                        .content_type("text/html")
                        .body(template),
                    Err(err) => HttpResponse::InternalServerError()
                        .content_type("text/html")
                        .body(format!(
                            "<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load blog article page: {}</span>",
                            err.to_string()
                        )),
                }
            }),
        );
}
