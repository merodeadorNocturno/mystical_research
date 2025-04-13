use crate::db::blog_db::BlogDB;
use crate::db::config_db::Database;
use crate::models::{
    blog_model::BlogPreview,
    general_model::PageType,
    index_model::{IndexPage, TitleError},
    mock::{mock_header_data, mock_index_body, mock_index_linked_data},
};
use crate::utils::{
    env_utils::*,
    fs_utils::{read_hbs_template, register_templates},
    general_utils::trim_to_words,
};
use actix_web::{
    web::{get, Data, ServiceConfig},
    HttpResponse,
};
use handlebars::{Handlebars, RenderError};
use log::error;
use serde_json::json;
use std::path::Path;

async fn index_html(db: &Data<Database>) -> Result<String, RenderError> {
    let mut handlebars = Handlebars::new();
    let PageConfiguration { template_path, .. } = set_env_urls();

    let this_path = Path::new(&template_path);

    register_templates(this_path, &mut handlebars);
    let index_hbs = "index/index";

    let section_template = match read_hbs_template(index_hbs) {
        Ok(contents) => contents,
        Err(err) => {
            error!(
                "Failed to render contents for edit title:: {}",
                err.to_string()
            );
            TitleError::new(err.to_string()).error
        }
    };

    let linked_data = mock_index_linked_data();
    let body = mock_index_body();
    let header = mock_header_data();
    // let mut featured = Vec::new();
    let number_of_records: usize = 3;

    let result = Database::find_random_articles(db, Some(number_of_records)).await;

    let featured_results = result.unwrap_or_default();

    let mut featured: Vec<BlogPreview> = Vec::new();
    for this_feature in featured_results {
        featured.push(BlogPreview {
            title: this_feature.title.unwrap(),
            summary: format!("{}...", trim_to_words(&this_feature.summary.unwrap(), 16)),
            image_url: this_feature.image_urls.unwrap(),
            slug: this_feature.slug.unwrap(),
        })
    }

    let section_template = handlebars.render_template(
        &section_template,
        &json!(&IndexPage {
            body,
            linked_data,
            featured,
            header,
            section: PageType::Home,
        }),
    )?;
    Ok(section_template)
}

pub fn index_controller(cfg: &mut ServiceConfig) {
    cfg.route(
      "/",
      get().to(|db: Data<Database>| async move {
        let mr_help_template = index_html(&db).await;
        match mr_help_template {
            Ok(template) => HttpResponse::Ok()
              .content_type("text/html")
              .append_header(("HX-Trigger", "help_table"))
             .body(template),
            Err(err) => HttpResponse::InternalServerError()
              .content_type("text/html")
              .append_header(("HX-Trigger", "help_table"))
              .body(format!("<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load title: {}</span>",
              err)),
        }
    }));
}
