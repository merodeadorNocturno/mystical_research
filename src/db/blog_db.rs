use crate::db::config_db::{check_field, Database};
use crate::models::blog_model::BlogArticle;
use crate::utils::crud_utils::*;
use crate::utils::general_utils::{create_or_conditional, get_uuid};
use actix_web::web::Data;
use async_trait::async_trait;
use chrono::Local;
use log::error;
// use surrealdb::{opt::PatchOp, Datetime, Error,};

const BLOG_TABLE: &str = "blog_article";

#[async_trait]
pub trait BlogDB {
    async fn find_all(db: &Data<Database>) -> Option<Vec<BlogArticle>>;
    async fn find_one(db: &Data<Database>, id: String) -> Option<BlogArticle>;
    async fn add_one(db: &Data<Database>, new_blog_article: BlogArticle) -> Option<BlogArticle>;
    async fn update_one(db: &Data<Database>, blog: BlogArticle) -> Option<BlogArticle>;
    async fn find_all_including_deleted(db: &Data<Database>) -> Option<Vec<BlogArticle>>;
    async fn find_all_deleted(db: &Data<Database>) -> Option<Vec<BlogArticle>>;
    async fn delete_one(db: &Data<Database>, id: String) -> Option<Vec<BlogArticle>>;
    async fn search_content(db: &Data<Database>, search_term: String) -> Option<Vec<BlogArticle>>;
}

#[async_trait]
impl BlogDB for Database {
    async fn find_all(db: &Data<Database>) -> Option<Vec<BlogArticle>> {
        if let Err(e) = check_field(db, BLOG_TABLE, "title", None) {
            error!("Schema validation error: {}", e);
            return None;
        }

        util_find_active_records(db, BLOG_TABLE).await
    }

    async fn find_one(db: &Data<Database>, id: String) -> Option<BlogArticle> {
        util_find_one(db, id, BLOG_TABLE).await
    }

    async fn add_one(db: &Data<Database>, new_blog_article: BlogArticle) -> Option<BlogArticle> {
        util_add_one(db, new_blog_article, BLOG_TABLE).await
    }

    async fn update_one(db: &Data<Database>, blog_article: BlogArticle) -> Option<BlogArticle> {
        util_update_record(db, blog_article, BLOG_TABLE).await
    }

    async fn find_all_including_deleted(db: &Data<Database>) -> Option<Vec<BlogArticle>> {
        util_find_all(&db, BLOG_TABLE).await
    }

    async fn find_all_deleted(db: &Data<Database>) -> Option<Vec<BlogArticle>> {
        util_query_deleted(&db, BLOG_TABLE).await
    }

    async fn delete_one(db: &Data<Database>, id: String) -> Option<Vec<BlogArticle>> {
        util_remove_record(&db, id, BLOG_TABLE).await
    }

    async fn search_content(db: &Data<Database>, search_term: String) -> Option<Vec<BlogArticle>> {
        let fields = vec![
            "summary".to_string(),
            "keywords".to_string(),
            "table_of_contents".to_string(),
            "author".to_string(),
        ];
        let search_fields = create_or_conditional(&search_term, fields);
        util_fulltext_search(&db, BLOG_TABLE, &search_fields).await
    }
}
