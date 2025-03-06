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
use std::{path::Path, u64::MAX};

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

async fn load_html(section: &str) -> Result<String, RenderError> {
    let mut handlebars = Handlebars::new();
    let this_path = Path::new("./src/static");

    register_templates(this_path, &mut handlebars);

    let section_template = match read_hbs_template(section) {
        Ok(contents) => contents,
        Err(err) => {
            error!(
                "Failed to render contents for edit title:: {}",
                err.to_string()
            );
            TitleError::new(err.to_string()).error
        }
    };

    let default_env = set_env_urls();
    let section_template = handlebars.render_template(&section_template, &json!(default_env))?;
    Ok(section_template)
}

// pub fn serve_parsed_html(cfg: &mut ServiceConfig) {
//     cfg.service(get("/parsed_html").to(load_html));
// }
