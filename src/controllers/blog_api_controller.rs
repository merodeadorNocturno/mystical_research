use crate::db::blog_db::BlogDB;
use crate::db::config_db::Database;
use crate::models::{
    ai_model::{AiResponse, BlogStructure, GenerateContentResponse},
    blog_model::BlogArticle,
    general_model::PageType,
};
use crate::utils::{
    ai_utils::{create_ai_request, generate_content},
    general_utils::generate_slug_with_random_suffix,
    response_utils::create_blog_structure_from_response,
};
use actix_web::{
    error::ErrorNotFound,
    get,
    http::StatusCode,
    // patch, post,
    web::{
        Data,
        // Json,
        Path,
        Query,
        ServiceConfig,
    },
    HttpResponse,
};
use log::error;
use serde::Deserialize;
// use serde_json::to_string;
// use validator::Validate;

#[derive(Debug, Deserialize)]
struct SearchQuery {
    q: String, // The search query will be in a parameter named 'q'
}

#[get("/blogs/search")]
#[tracing::instrument(name = "Show Blog Articles", skip(db))]
async fn blogs_search(
    db: Data<Database>,
    query: Query<SearchQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    // Use actix_web::Error for clarity
    let search_term = String::from(&query.q);
    let blog_articles = Database::search_content(&db, search_term).await;
    match blog_articles {
        Some(articles_found) => Ok(HttpResponse::Ok()
            .status(StatusCode::OK)
            .json(articles_found)),
        None => Err(ErrorNotFound("No articles found")), // Return the ErrorNotFound *value*
    }
}

#[get("/blogs/ai/creator")]
#[tracing::instrument(name = "Show Blog Articles", skip(db))]
async fn blogs_ai_creator(db: Data<Database>) -> Result<HttpResponse, actix_web::Error> {
    let mut ai_response: GenerateContentResponse = GenerateContentResponse::new(vec![]);
    let mut _ai_error: String = String::new();

    let token_string = "Write a dialogue between two long-time friends passionately discussing music in the Classical and Romantics periods.
        One friend is a PhD in Literature, whose expertise lies in analyzing the literature and artistic movements of the Romantic period.
        The other friend is a seasoned musician with decades of hands-on experience in directing orchestras to play Classical period pieces.
        Their conversation unfolds on a stage in an history and aesthetics conference sponsored by a famous british university,
        and they explore themes such as the movement's historical context,
        its relationship with each other, public perception,
        and the interplay between social movements and aesthetics.
        Despite their differing perspectives,
        their mutual respect and camaraderie allow for a thoughtful and nuanced exchange,
        peppered with humor, witty banter, and personal anecdotes.
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

    let surrealdb_now: surrealdb::Datetime = surrealdb::Datetime::from(chrono::Utc::now());

    let ai_response_from_google = &ai_response.candidates[0].content.parts[0].text;

    let blog_structure: BlogStructure =
        create_blog_structure_from_response(ai_response_from_google);
    let title = blog_structure.title;

    let blog_article: BlogArticle = BlogArticle::builder()
        .title(String::from(&title))
        .summary(blog_structure.summary)
        .slug(generate_slug_with_random_suffix(&title))
        .content(blog_structure.content)
        .keywords(blog_structure.keywords.join(","))
        .table_of_contents(blog_structure.table_of_contents.join(","))
        .page_type(PageType::BlogArticle)
        .deleted(false)
        .author("BaggiE BomziE".to_string())
        .image_urls("article1.png".to_string())
        .created_at(&surrealdb_now)
        .updated_at(&surrealdb_now)
        .published_at(&surrealdb_now)
        .build();

    let posted_blog = Database::add_one(&db, blog_article).await;

    match posted_blog {
        Some(new_blog) => Ok(HttpResponse::Ok()
            .status(StatusCode::CREATED)
            .json(new_blog)),
        None => {
            error!("Failed to post blog article");
            Ok(HttpResponse::InternalServerError()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .json("Failed to post blog article"))
        }
    }
}

#[get("/blogs")]
#[tracing::instrument(name = "Show Blog Articles", skip(db))]
async fn blogs(db: Data<Database>) -> Result<HttpResponse, actix_web::Error> {
    let active_blogs = Database::find_all(&db).await;

    match active_blogs {
        Some(blogs) => Ok(HttpResponse::Ok().status(StatusCode::OK).json(blogs)),
        None => {
            error!("Failed to fetch active blogs");
            Ok(HttpResponse::InternalServerError()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .json("Failed to fetch active blogs"))
        }
    }
}

#[get("/blogs/{id}")]
#[tracing::instrument(
    name = "Get One Blog Article",    // 1. Descriptive span name
    skip(db),                 // 2. Skip logging the database connection pool
    fields(thing_id = %id)    // 3. Record the 'id' path parameter
)]
async fn blogs_id(db: Data<Database>, id: Path<String>) -> Result<HttpResponse, actix_web::Error> {
    let blog_article = Database::find_one(&db, id.to_string()).await;

    match blog_article {
        Some(blog) => Ok(HttpResponse::Ok().status(StatusCode::OK).json(blog)),
        None => {
            error!("Failed to fetch blog article");
            Ok(HttpResponse::NotFound()
                .status(StatusCode::NOT_FOUND)
                .json("Blog article not found"))
        }
    }
}

#[get("/blogs/article/{slug}")]
#[tracing::instrument(name = "Get Blog Article By Slug", skip(db), fields(slug = %slug))]
async fn blogs_article_slug(
    db: Data<Database>,
    slug: Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let blog_article = Database::search_slug_id(&db, slug.to_string()).await;

    match blog_article {
        Some(blog) => Ok(HttpResponse::Ok().status(StatusCode::OK).json(blog)),
        None => {
            error!("Failed to fetch blog article");
            Ok(HttpResponse::NotFound()
                .status(StatusCode::NOT_FOUND)
                .json("Blog article not found"))
        }
    }
}

pub fn blog_api_controller(cfg: &mut ServiceConfig) {
    cfg.service(blogs);
    cfg.service(blogs_id);
    cfg.service(blogs_ai_creator);
    cfg.service(blogs_article_slug);
    cfg.service(blogs_search);
}
