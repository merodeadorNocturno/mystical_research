use crate::models::mock::mock_about_home_page_object;
use crate::utils::{
    env_utils::*,
    fs_utils::{read_hbs_template, register_templates},
};
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

async fn htmx_about() -> Result<String, RenderError> {
    let mut handlebars = Handlebars::new();
    let PageConfiguration { template_path, .. } = set_env_urls();

    let this_path = Path::new(&template_path);

    register_templates(this_path, &mut handlebars);
    let about_hbs = "about/about";

    let section_template = match read_hbs_template(&about_hbs) {
        Ok(contents) => contents,
        Err(err) => {
            error!(
                "Failed to render contents for edit title:: {}",
                err.to_string()
            );
            TitleError::new(err.to_string()).error
        }
    };

    let section_template = handlebars.render_template(&section_template, &json!({}))?;
    Ok(section_template)
}

async fn about_html() -> Result<String, RenderError> {
    let mut handlebars = Handlebars::new();
    let PageConfiguration { template_path, .. } = set_env_urls();

    let this_path = Path::new(&template_path);

    register_templates(this_path, &mut handlebars);
    let topics_index_hbs = "index/index";
    let section_template = match read_hbs_template(&topics_index_hbs) {
        Ok(contents) => contents,
        Err(err) => {
            error!(
                "Failed to render contents for edit title:: {}",
                err.to_string()
            );
            TitleError::new(err.to_string()).error
        }
    };

    let section_template = match handlebars
        .render_template(&section_template, &json!(&mock_about_home_page_object()))
    {
        Ok(template) => template,
        Err(err) => {
            error!(
                "Failed to render contents for edit title:: {}",
                err.to_string()
            );
            TitleError::new(err.to_string()).error
        }
    };
    Ok(section_template)
}

pub fn about_controller(cfg: &mut ServiceConfig) {
    cfg.route(
      "/htmx/about",
      get().to(|| async move {
        let mr_help_template = htmx_about().await;
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
    cfg.route(
      "/about.html",
      get().to(|| async move {
        let mr_help_template = about_html().await;
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
