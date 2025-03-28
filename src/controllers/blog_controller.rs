use crate::db::blog_db::BlogDB;
use crate::db::config_db::Database;
use crate::models::{
    ai_model::{AiResponse, GenerateContentResponse},
    blog_model::{BlogArticle, BlogPreview},
};
use crate::utils::{
    ai_utils::{create_ai_request, generate_content},
    env_utils::{set_env_urls, PageConfiguration},
    fs_utils::{read_hbs_template, register_templates},
    general_utils::trim_to_words,
    response_utils::create_blog_structure_from_response,
};
use actix_web::{
    web::{get, Data, ServiceConfig},
    HttpResponse,
};
use handlebars::{Handlebars, RenderError};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::Path;
// use tracing::warn;

#[derive(Debug, Deserialize, Serialize)]
struct TitleError {
    pub error: String,
}

impl TitleError {
    pub fn new(error: String) -> TitleError {
        TitleError { error }
    }
}

async fn load_blog_article() -> Result<String, RenderError> {
    let mut handlebars = Handlebars::new();
    let PageConfiguration { template_path, .. } = set_env_urls();

    let this_path = Path::new(&template_path);

    let mut ai_response: GenerateContentResponse = GenerateContentResponse::new(vec![]);
    let mut _ai_error: String = String::new();

    register_templates(this_path, &mut handlebars);
    let blog_article_hbs = "blog/blog_article_og";

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

    // let token_string =
    //     "Pretend you have a PhD in ethics. Write a blog article on the ethics and contradictions of theology and religious practices.
    //     Use the first paragraph for the title.
    // Use the second paragraph for table of contents, separated by commas.
    // Use the third paragraph for an abstract summary.
    // Use the last paragraph for a comma separated list of keywords.";
    let token_string = "You are an expert in philology, linguistics, and the Talmud.
      Please write a 13 paragraph essay trying to map and connect each of the sefiroth to each of the choirs of angels in christianity.
      Please try to create different hypotheses on how the sefiroth and choirs of angels can be connected. Try exploring gematria and atbash ciphers.
      Also, try to use real knowledge of the Talmud and the Kabbalah and biblical sources that are relevant to the choirs of angels.
  Avoid making statements that are not supported by evidence or research. e.g. 'The cool touch of moonlight on the skin' is not acceptable.
      Avoid making statements that are not supported by evidence or research. e.g. 'The cool touch of moonlight on the skin' is not acceptable.
      Use the first paragraph for the title.
      Use the second paragraph for table of contents, separated by commas.
      Use the third paragraph for an abstract summary.
      Use the last paragraph for a comma separated list of keywords.";

    let ai_request = create_ai_request(token_string);

    match generate_content(ai_request).await {
        Ok(AiResponse {
            response_body: rb, ..
        }) => {
            ai_response = rb;
        }
        Err(err) => {
            error!(
                "Failed to generate content for blog article: {}",
                err.to_string()
            );
            _ai_error = format!("Failed to generate article content: {}", err.to_string());
        }
    };

    let ai_response_from_google = &ai_response.candidates[0].content.parts[0].text;
    let blog_structure = create_blog_structure_from_response(ai_response_from_google);
    let section_template = handlebars.render_template(&section_template, &json!(blog_structure))?;

    Ok(section_template)
}

async fn load_blog_html(db: &Data<Database>) -> Result<String, RenderError> {
    let PageConfiguration { template_path, .. } = set_env_urls();

    let mut handlebars = Handlebars::new();
    let this_path = Path::new(&template_path);

    register_templates(this_path, &mut handlebars);
    let blog_home_hbs = "index/index";

    let articles_opt = Database::find_all(db).await;
    let blog_previews: Vec<BlogPreview> = get_blog_articles_from_db(articles_opt);

    let blog_home_template = match read_hbs_template(&blog_home_hbs) {
        Ok(template) => template,
        Err(err) => {
            eprintln!("Failed to read blog home template: {}", err);
            String::new()
        }
    };

    let context_data = json!({ "posts": blog_previews, "section": "BlogHome" });

    let section_blog_home_template =
        match handlebars.render_template(&blog_home_template, &context_data) {
            Ok(template) => template,
            Err(err) => {
                eprintln!("Failed to render blog home template: {}", err);
                String::new()
            }
        };

    Ok(section_blog_home_template)
}

async fn load_blog_index_from_db(db: &Data<Database>) -> Result<String, RenderError> {
    let mut handlebars = Handlebars::new();
    let PageConfiguration { template_path, .. } = set_env_urls();

    let this_path = Path::new(&template_path);

    register_templates(this_path, &mut handlebars);
    let blog_home_hbs_path = "blog/blog_home";

    let articles_opt = Database::find_all(db).await;
    let blog_previews: Vec<BlogPreview> = get_blog_articles_from_db(articles_opt);

    let template_content = match read_hbs_template(&blog_home_hbs_path) {
        Ok(contents) => contents,
        Err(err) => {
            error!("Failed to read blog home template: {}", err);
            return Err(RenderError::from(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Template not found: {}", blog_home_hbs_path),
            )));
        }
    };

    let context_data = json!({"posts": blog_previews,});

    match handlebars.render_template(&template_content, &context_data) {
        Ok(rendered_html) => Ok(rendered_html),
        Err(e) => {
            error!("Failed to render blog home template: {}", e);
            Err(e) // Propagate the render error
        }
    }
}

pub fn blog_home_html(cfg: &mut ServiceConfig) {
    cfg.route(
        "/blog",
        get().to(|db: Data<Database>| async move {
            match load_blog_index_from_db(&db).await {
                Ok(template) => HttpResponse::Ok()
                    .content_type("text/html")
                    .body(template),
                Err(err) => {
                    error!("Error rendering blog index with DB: {}", err);
                    HttpResponse::InternalServerError()
                    .content_type("text/html")
                    .body(format!(
                        "<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load blog home page: {}</span>",
                        err.to_string()
                    ))
                }
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

    cfg.route(
      "/blog_home.html",
      get().to(|db: Data<Database>| async move {
        let blog_home_template = load_blog_html(&db).await;
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
      })
    );
}

fn get_blog_articles_from_db(articles: Option<Vec<BlogArticle>>) -> Vec<BlogPreview> {
    match articles {
        Some(articles) => {
            info!("Fetched {} blog articles from DB", articles.len());
            articles
                .into_iter()
                .filter_map(|article| {
                    match (
                        article.id,
                        article.title,
                        article.summary,
                        article.image_urls,
                        article.slug,
                    ) {
                        (Some(id), Some(title), Some(summary), Some(image_url), Some(slug)) => {
                            let id_str = id.id.to_string();
                            Some(BlogPreview {
                                id: id_str,
                                image_url,
                                slug,
                                summary: format!(
                                    "{}...",
                                    String::from(trim_to_words(&summary, 14))
                                ),
                                title,
                            })
                        }
                        _ => {
                            warn!("Invalid blog article data");
                            None
                        }
                    }
                })
                .collect()
        }
        None => {
            error!("Failed to fetch blog articles from DB");
            Vec::new()
        }
    }
}
