use crate::db::blog_db::BlogDB;
use crate::db::config_db::Database;
use crate::models::blog_model::{BlogArticle, BlogPreview};
use crate::utils::{
    env_utils::{set_env_urls, PageConfiguration},
    fs_utils::{read_hbs_template, register_templates},
    general_utils::{string_to_vec_string, trim_to_words},
    schema_markup::schema_markup_blog_article,
};
use actix_web::{
    web::{get, Data, Path, ServiceConfig},
    HttpRequest, HttpResponse,
};
use handlebars::{Handlebars, RenderError};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::json;
// use tracing::warn;

#[derive(Debug, Deserialize, Serialize)]
struct TitleError {
    pub error: String,
}

async fn blog_home_html(db: &Data<Database>) -> Result<String, RenderError> {
    let PageConfiguration { template_path, .. } = set_env_urls();

    let mut handlebars = Handlebars::new();
    let this_path = std::path::Path::new(&template_path);

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

async fn blog_article_slug(
    article_slug: Path<String>,
    db: &Data<Database>,
) -> Result<String, RenderError> {
    let PageConfiguration { template_path, .. } = set_env_urls();
    let search_term = article_slug.into_inner();
    info!("Rendering blog article slug {}", search_term);

    let mut handlebars = Handlebars::new();
    let this_path = std::path::Path::new(&template_path);

    register_templates(this_path, &mut handlebars);
    let blog_article_hbs = "index/index";

    let article_result = Database::search_slug_id(db, search_term).await;

    let blog_article_template = match read_hbs_template(&blog_article_hbs) {
        Ok(template) => template,
        Err(err) => {
            error!("Failed to read blog article template: {}", err);
            return Err(RenderError::from(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Template not found: {}", blog_article_hbs),
            )));
        }
    };

    let this_article = match article_result {
        Some(article) => article,
        None => vec![],
    };

    let mut context_data = json!({});

    if this_article.len() > 0 {
        let table_of_contents =
            string_to_vec_string(this_article[0].table_of_contents.clone().unwrap());
        let keywords = string_to_vec_string(this_article[0].keywords.clone().unwrap());

        context_data = json!({
            "article": this_article[0],
            "table_of_contents": table_of_contents,
            "keywords": keywords,
            "section": this_article[0].page_type.clone().unwrap(),
            "header": {
                "canonical_url": this_article[0].slug.clone().unwrap(),
                "site_title": this_article[0].title.clone().unwrap(),
                "site_description": this_article[0].summary.clone().unwrap(),
                "logo_url": this_article[0].image_urls.clone().unwrap(),
            },
            "schema_markup": schema_markup_blog_article(&this_article[0]),
        });
    }

    match handlebars.render_template(&blog_article_template, &context_data) {
        Ok(rendered) => Ok(rendered),
        Err(err) => {
            error!("Failed to render blog article template: {}", err);
            return Err(RenderError::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to render template: {}", blog_article_hbs),
            )));
        }
    }
}

async fn htmx_blog(db: &Data<Database>) -> Result<String, RenderError> {
    let mut handlebars = Handlebars::new();
    let PageConfiguration { template_path, .. } = set_env_urls();

    let this_path = std::path::Path::new(&template_path);

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

async fn htmx_blog_article_slug(
    query_string: Path<String>,
    db: &Data<Database>,
) -> Result<String, RenderError> {
    let search_term = query_string.into_inner();
    let mut handlebars = Handlebars::new();
    let PageConfiguration { template_path, .. } = set_env_urls();

    let this_path = std::path::Path::new(&template_path);
    register_templates(this_path, &mut handlebars);
    let blog_article_hbs_path = "blog/blog_article_og";

    let article = Database::search_slug_id(db, search_term).await;

    let template_content = match read_hbs_template(&blog_article_hbs_path) {
        Ok(contents) => contents,
        Err(err) => {
            error!("Failed to read blog article template: {}", err);
            return Err(RenderError::from(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Template not found: {}", blog_article_hbs_path),
            )));
        }
    };

    let this_article = match article {
        Some(article_in_db) => article_in_db,
        None => vec![],
    };

    let mut context_data = json!({});

    if this_article.len() > 0 {
        let table_of_contents =
            string_to_vec_string(this_article[0].table_of_contents.clone().unwrap());
        let keywords = string_to_vec_string(this_article[0].keywords.clone().unwrap());

        context_data = json!({
            "article": this_article[0],
            "table_of_contents": table_of_contents,
            "keywords": keywords
        });
    }

    match handlebars.render_template(&template_content, &context_data) {
        Ok(rendered_html) => Ok(rendered_html),
        Err(e) => {
            error!("Failed to render blog article template: {}", e);
            Err(e)
        }
    }
}

pub fn blog_html_controller(cfg: &mut ServiceConfig) {
    cfg.route(
        "/htmx/blog",
        get().to(|db: Data<Database>| async move {
            match htmx_blog(&db).await {
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
      "/blog_home.html",
      get().to(|db: Data<Database>| async move {
        let blog_home_template = blog_home_html(&db).await;
        match blog_home_template {
            Ok(template) => HttpResponse::Ok()
                .content_type("text/html")
                .body(template),
            Err(err) => HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(format!(
                    "<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load blog home page: {}</span>",
                    err.to_string()
                )
            ),
        }
      })
    );

    cfg.route(
        "/htmx/blog/article/{slug}",
        get().to(|_req: HttpRequest, slug, db: Data<Database>| async move {
            let blog_article_template = htmx_blog_article_slug(slug, &db).await;
            match blog_article_template {
                Ok(article) => HttpResponse::Ok()
                    .content_type("text/html")
                    .body(article),
                Err(err) => HttpResponse::InternalServerError()
                    .content_type("text/html")
                    .append_header(("HX-Trigger", "error_enterprise_table"))
                    .body(format!(
                        "<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load blog article: {}</span>",
                        err.to_string()
                    )),
            }
        })
    );

    cfg.route(
        "/blog/article/{slug}",
        get().to(
            |_req: HttpRequest, slug, db: Data<Database>| async move {
                let blog_article_template = blog_article_slug(slug, &db).await;
                match blog_article_template {
                    Ok(article) => HttpResponse::Ok()
                        .content_type("text/html")
                        .body(article),
                    Err(err) => HttpResponse::InternalServerError()
                        .content_type("text/html")
                        .append_header(("HX-Trigger", "error_enterprise_table"))
                        .body(format!(
                            "<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load blog article: {}</span>",
                            err.to_string()
                        )),
                }
            }
        )
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
