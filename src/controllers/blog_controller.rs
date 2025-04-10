use crate::db::blog_db::BlogDB;
use crate::db::config_db::Database;
use crate::models::{
    blog_model::{BlogArticle, BlogPreview},
    general_model::SearchQuery,
};
use crate::utils::{
    env_utils::{set_env_urls, PageConfiguration},
    fs_utils::{read_hbs_template, register_templates},
    general_utils::{string_to_vec_string, trim_to_words},
    linked_data::linked_data_blog_article,
};
use actix_web::{
    web::{get, Data, Path, Query, ServiceConfig},
    HttpRequest, HttpResponse,
};
use handlebars::{Handlebars, RenderError};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
struct TitleError {
    pub error: String,
}

async fn blog_home_html(
    query: Query<BlogHomeQuery>,
    db: &Data<Database>,
) -> Result<String, RenderError> {
    let PageConfiguration { template_path, .. } = set_env_urls();
    let number_of_results: Option<usize> = query.r;

    let mut handlebars = Handlebars::new();
    let this_path = std::path::Path::new(&template_path);

    register_templates(this_path, &mut handlebars);
    let blog_home_hbs = "index/index";

    let articles_opt = match number_of_results {
        Some(num) => Database::find_all(db, Some(num)).await,
        None => Database::find_all(db, None).await,
    };
    let blog_previews: Vec<BlogPreview> = get_blog_articles_from_db(articles_opt);

    let blog_home_template = match read_hbs_template(blog_home_hbs) {
        Ok(template) => template,
        Err(err) => {
            eprintln!("Failed to read blog home template: {}", err);
            String::new()
        }
    };

    let context_data = json!({
        "posts": blog_previews,
        "section": "BlogHome",
        "linked_data": {
            "description": "A collection of mystical and artistic explorations.",
            "logo_url": "/static/img/hero-bg.png",
            "blog_posts": &blog_previews,
    } });

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

    let blog_article_template = match read_hbs_template(blog_article_hbs) {
        Ok(template) => template,
        Err(err) => {
            error!("Failed to read blog article template: {}", err);
            return Err(RenderError::from(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Template not found: {}", blog_article_hbs),
            )));
        }
    };

    let this_article = article_result.unwrap_or_default();
    //     Some(article) => article,
    //     None => vec![],
    // };

    let mut context_data = json!({});

    if !this_article.is_empty() {
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
            "linked_data": linked_data_blog_article(&this_article[0]),
        });
    }

    match handlebars.render_template(&blog_article_template, &context_data) {
        Ok(rendered) => Ok(rendered),
        Err(err) => {
            error!("Failed to render blog article template: {}", err);
            Err(RenderError::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to render template: {}", blog_article_hbs),
            )))
        }
    }
}

async fn htmx_blog(
    query: Query<BlogHomeQuery>,
    db: &Data<Database>,
) -> Result<String, RenderError> {
    let mut handlebars = Handlebars::new();
    let PageConfiguration { template_path, .. } = set_env_urls();

    let this_path = std::path::Path::new(&template_path);

    register_templates(this_path, &mut handlebars);
    let blog_home_hbs_path = "blog/blog_home";

    let articles_opt = match query.r {
        Some(number) => Database::find_all(db, Some(number)).await,
        None => Database::find_all(db, None).await,
    };

    let blog_previews: Vec<BlogPreview> = get_blog_articles_from_db(articles_opt);

    let template_content = match read_hbs_template(blog_home_hbs_path) {
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

    let template_content = match read_hbs_template(blog_article_hbs_path) {
        Ok(contents) => contents,
        Err(err) => {
            error!("Failed to read blog article template: {}", err);
            return Err(RenderError::from(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Template not found: {}", blog_article_hbs_path),
            )));
        }
    };

    let this_article = article.unwrap_or_default();

    let mut context_data = json!({});

    if !this_article.is_empty() {
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

async fn htmx_blog_search(
    query: Query<SearchQuery>,
    db: &Data<Database>,
) -> Result<String, RenderError> {
    let PageConfiguration { template_path, .. } = set_env_urls();

    let mut handlebars = Handlebars::new();
    let this_path = std::path::Path::new(&template_path);

    register_templates(this_path, &mut handlebars);
    let blog_grid_hbs = "blog/_blog_post_grid";

    let search_term = query.q.clone().unwrap_or_default(); // Get the search term

    let search_result: Option<Vec<BlogArticle>> = if !search_term.is_empty() {
        Database::search_content(db, search_term.clone()).await // Pass cloned term
    } else {
        None // Or maybe return latest posts if search is empty? Decide behavior.
             // Let's assume empty search term means no results for now.
    };

    let blog_previews: Vec<BlogPreview> = get_blog_articles_from_db(search_result);

    let template_content = match read_hbs_template(blog_grid_hbs) {
        Ok(contents) => contents,
        Err(err) => {
            error!("Failed to read blog grid partial template: {}", err);
            return Err(RenderError::from(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Template not found: {}", &blog_grid_hbs), // <<< Use correct path variable
            )));
        }
    };

    let context_data = json!(
    {
        "posts": blog_previews,
        "search_term": if search_term.is_empty() {
            None
        } else {
            Some(&search_term)
        }
    });

    match handlebars.render_template(&template_content, &context_data) {
        Ok(rendered_html) => Ok(rendered_html),
        Err(e) => {
            error!("Failed to render blog home template: {}", e);
            Err(e) // Propagate the render error
        }
    }
}

pub fn blog_html_controller(cfg: &mut ServiceConfig) {
    cfg.route(
        "/blog/article/{slug}",
        get().to(|_req: HttpRequest, slug, db: Data<Database>| async move {
            let blog_article_template = blog_article_slug(slug, &db).await;
            respond_to_html_result(
                blog_article_template,
                "There was an error when trying to retrieve the blog article",
            )
        }),
    );

    cfg.route(
        "/blog_home.html",
        get().to(
            |query: Query<BlogHomeQuery>, db: Data<Database>| async move {
                let blog_home_template = blog_home_html(query, &db).await;
                respond_to_html_result(blog_home_template, "Error retrieving blog home page")
            },
        ),
    );

    cfg.route(
        "/htmx/blog",
        get().to(
            |query: Query<BlogHomeQuery>, db: Data<Database>| async move {
                let htmx_result = htmx_blog(query, &db).await;
                respond_to_html_result(htmx_result, "Failed to retrieve htmx blog")
            },
        ),
    );

    cfg.route(
        "/htmx/blog/article/{slug}",
        get().to(|_req: HttpRequest, slug, db: Data<Database>| async move {
            let blog_article_template = htmx_blog_article_slug(slug, &db).await;
            respond_to_html_result(
                blog_article_template,
                "Could not retrieve htmx blog article",
            )
        }),
    );

    cfg.route(
        "/htmx/blog/search",
        get().to(|query: Query<SearchQuery>, db: Data<Database>| async move {
            let htmx_result = htmx_blog_search(query, &db).await;
            respond_to_html_result(
                htmx_result,
                "An error occurred when consulting the database",
            )
        }),
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
                        article.title,
                        article.summary,
                        article.image_urls,
                        article.slug,
                    ) {
                        (Some(title), Some(summary), Some(image_url), Some(slug)) => {
                            Some(BlogPreview {
                                image_url,
                                slug,
                                summary: format!("{}...", trim_to_words(&summary, 14)),
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

#[derive(Deserialize, Debug)] // <-- Must derive Deserialize
struct BlogHomeQuery {
    r: Option<usize>,
}

fn respond_to_html_result(
    result: Result<String, RenderError>,
    error_message_prefix: &str,
) -> HttpResponse {
    match result {
        Ok(template) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(template),
        Err(err) => {
            error!("{}: {}", error_message_prefix, err);
            HttpResponse::InternalServerError()
                .content_type("text/html; charset=utf-8")
                .body(format!(
                    "<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>{}: {}</span>",
                    error_message_prefix,
                    err
                ))
        }
    }
}
