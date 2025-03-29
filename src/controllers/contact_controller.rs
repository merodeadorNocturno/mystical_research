use crate::models::mock::mock_contact_home_page_object;
use crate::utils::{
    env_utils::{set_env_urls, PageConfiguration},
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
struct TitleError {
    pub error: String,
}

impl TitleError {
    pub fn new(error: String) -> TitleError {
        TitleError { error }
    }
}

async fn htmx_contact() -> Result<String, RenderError> {
    let mut handlebars = Handlebars::new();
    let PageConfiguration { template_path, .. } = set_env_urls();

    let this_path = Path::new(&template_path);

    register_templates(this_path, &mut handlebars);
    let contact_hbs = "contact/contact";

    let section_template = match read_hbs_template(&contact_hbs) {
        Ok(contents) => contents,
        Err(err) => {
            error!(
                "Failed to render contents for contact page: {}",
                err.to_string()
            );
            TitleError::new(err.to_string()).error
        }
    };

    let section_template = handlebars.render_template(&section_template, &json!({}))?;
    Ok(section_template)
}

async fn contact_html() -> Result<String, RenderError> {
    let PageConfiguration { template_path, .. } = set_env_urls();

    let mut handlebars = Handlebars::new();
    let this_path = Path::new(&template_path);

    register_templates(this_path, &mut handlebars);
    let contact_hbs = "index/index";

    let contact_home_template = match read_hbs_template(&contact_hbs) {
        Ok(contents) => contents,
        Err(err) => {
            error!(
                "Failed to render contents for contact page: {}",
                err.to_string()
            );
            TitleError::new(err.to_string()).error
        }
    };

    let section_template = match handlebars.render_template(
        &contact_home_template,
        &json!(&mock_contact_home_page_object()),
    ) {
        Ok(template) => template,
        Err(err) => {
            error!(
                "Failed to render contents for contact page: {}",
                err.to_string()
            );
            TitleError::new(err.to_string()).error
        }
    };
    Ok(section_template)
}

pub fn contact_controller(cfg: &mut ServiceConfig) {
    cfg.route(
        "/htmx/contact",
        get().to(|| async move {
            let contact_template = htmx_contact().await;
            match contact_template {
                Ok(template) => HttpResponse::Ok()
                    .content_type("text/html")
                    .body(template),
                Err(err) => HttpResponse::InternalServerError()
                    .content_type("text/html")
                    .body(format!(
                        "<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load contact page: {}</span>",
                        err.to_string()
                    )),
            }
        }),
    );

    cfg.route(
        "/contact.html",
        get().to(|| async move {
            let contact_home_template = contact_html().await;
            match contact_home_template {
                Ok(template) => HttpResponse::Ok()
                    .content_type("text/html")
                    .body(template),
                Err(err) => HttpResponse::InternalServerError()
                    .content_type("text/html")
                    .body(format!(
                        "<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load contact page: {}</span>",
                        err.to_string()
                    )),
            }
        }),
    );
}
