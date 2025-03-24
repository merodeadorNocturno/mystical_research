use crate::db::blog_db::BlogDB;
use crate::db::config_db::Database;
use crate::models::{
    ai_model::{AiResponse, BlogStructure, GenerateContentResponse},
    blog_model::BlogArticle,
    general_model::PageType,
};
use crate::utils::{
    ai_utils::{create_ai_request, generate_content},
    response_utils::create_blog_structure_from_response,
};
use actix_web::{
    delete,
    error::ErrorNotFound,
    get,
    http::StatusCode,
    patch, post,
    web::{Data, Json, Path, Query, ServiceConfig},
    HttpResponse,
};
use log::error;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize)]
struct SearchQuery {
    q: String, // The search query will be in a parameter named 'q'
}

#[get("/blogs/search")]
#[tracing::instrument(name = "Show Blog Articles", skip(db))]
async fn search_content(
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
async fn ai_creator(db: Data<Database>) -> Result<HttpResponse, actix_web::Error> {
    // Use actix_web::Error for clarity

    let mut ai_response: GenerateContentResponse = GenerateContentResponse::new(vec![]);
    let mut _ai_error: String = String::new();

    let token_string = "You have a PhD in biblical studies with a master's degree in Linguistics. Also you are a cat lover.
  Write a 10 paragraph essay titled
  'A Comparative Analysis of Domestic Cat Breeds and the Nine Choirs of Angels in Christian Angelology'
  Focus on the characteristics of each choir of Angels and the socially accepted behaviors of different cat breeds.
  The article should consider how the symbolic imagery of cats associated with the symbolism of angels in Christianity is interpreted in their physical characteristics and behaviors.
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

    // let now_system_time: SystemTime = SystemTime::from(Utc::now());
    let surrealdb_now: surrealdb::Datetime = surrealdb::Datetime::from(chrono::Utc::now());

    let ai_response_from_google = &ai_response.candidates[0].content.parts[0].text;

    let blog_structure: BlogStructure =
        create_blog_structure_from_response(ai_response_from_google);

    let blog_article: BlogArticle = BlogArticle::builder()
        .title(blog_structure.title)
        .summary(blog_structure.summary)
        .content(blog_structure.content)
        .keywords(blog_structure.keywords.join(","))
        .table_of_contents(blog_structure.table_of_contents.join(","))
        .page_type(PageType::BlogPost)
        .deleted(false)
        .author("BaggiE BomziE".to_string())
        .image_urls(String::from("article1.png, article2.png, article3.png"))
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
async fn all_active_blogs(db: Data<Database>) -> Result<HttpResponse, actix_web::Error> {
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

pub fn blog_api_routes(cfg: &mut ServiceConfig) {
    cfg.service(search_content);
    cfg.service(ai_creator);
    cfg.service(all_active_blogs);
}
