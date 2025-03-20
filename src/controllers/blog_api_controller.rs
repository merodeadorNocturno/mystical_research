use crate::db::blog_db::BlogDB;
use crate::db::config_db::Database;
use crate::models::{
    ai_model::{AiResponse, BlogStructure, GenerateContentResponse},
    blog_model::{BlogArticle, BlogArticleBuilder},
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
use chrono::Utc;
use log::{error, info};
use serde::Deserialize;
use std::time::SystemTime;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize)]
struct SearchQuery {
    q: String, // The search query will be in a parameter named 'q'
}

#[get("/blog/search")]
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

#[get("/blog/ai/creator")]
#[tracing::instrument(name = "Show Blog Articles", skip(db))]
async fn ai_creator(db: Data<Database>) -> Result<HttpResponse, actix_web::Error> {
    // Use actix_web::Error for clarity

    let mut ai_response: GenerateContentResponse = GenerateContentResponse::new(vec![]);
    let mut _ai_error: String = String::new();

    let token_string = "Pretend you have a PhD in philology and a master's degree in anthropology.
      Write an 6 paragraph essay titled
      'The Sensual Language of Moonlight: Dreams, Intuition, and the Mystical Unveiling.'
      Focus on the exploration of dreams, inner experience and emphasis on sensuality and mystical traditions,
      this article should examine how the sensory, sexual, and aesthetic qualities of moonlight are culturally linked to dreams,
      intuition, and mystical insights. It would explore anthropological accounts of how different societies interpret moonlit nights as conducive to altered states of consciousness, sensual awakenings, or encounters with the mystical.
      The article could consider how the symbolic imagery associated with the moon in dreams is understood as a form of mystical communication.
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

pub fn blog_api_routes(cfg: &mut ServiceConfig) {
    cfg.service(search_content);
    cfg.service(ai_creator);
}
