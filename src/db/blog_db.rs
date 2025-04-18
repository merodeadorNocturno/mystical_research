use std::ptr::null_mut;

use crate::db::config_db::Database;
use crate::models::blog_model::BlogArticle;
use crate::utils::crud_utils::*;
use crate::utils::general_utils::create_or_conditional;
use actix_web::web::Data;
use async_trait::async_trait;
// use surrealdb::{opt::PatchOp, Datetime, Error,};

const BLOG_TABLE: &str = "blog_article";

#[async_trait]
pub trait BlogDB {
    async fn find_all(
        db: &Data<Database>,
        number_of_records: Option<usize>,
    ) -> Option<Vec<BlogArticle>>;
    async fn find_one(db: &Data<Database>, id: String) -> Option<BlogArticle>;
    async fn add_one(db: &Data<Database>, new_blog_article: BlogArticle) -> Option<BlogArticle>;
    // async fn update_one(db: &Data<Database>, blog: BlogArticle) -> Option<BlogArticle>;
    // async fn find_all_including_deleted(db: &Data<Database>) -> Option<Vec<BlogArticle>>;
    // async fn find_all_deleted(db: &Data<Database>) -> Option<Vec<BlogArticle>>;
    // async fn delete_one(db: &Data<Database>, id: String) -> Option<Vec<BlogArticle>>;
    async fn search_content(db: &Data<Database>, search_term: String) -> Option<Vec<BlogArticle>>;
    async fn search_slug_id(db: &Data<Database>, search_term: String) -> Option<Vec<BlogArticle>>;
    async fn find_random_articles(
        db: &Data<Database>,
        number_of_records: Option<usize>,
    ) -> Option<Vec<BlogArticle>>;
    async fn get_number_of_articles(db: &Data<Database>) -> Option<u64>;
    async fn find_active_paginated(
        db: &Data<Database>,
        page: usize,
        page_size: usize,
    ) -> Option<Vec<BlogArticle>>;
}

#[async_trait]
impl BlogDB for Database {
    async fn find_all(
        db: &Data<Database>,
        number_of_records: Option<usize>,
    ) -> Option<Vec<BlogArticle>> {
        util_find_active_records(db, BLOG_TABLE, number_of_records).await
    }

    async fn find_one(db: &Data<Database>, id: String) -> Option<BlogArticle> {
        util_find_one(db, id, BLOG_TABLE).await
    }

    async fn add_one(db: &Data<Database>, new_blog_article: BlogArticle) -> Option<BlogArticle> {
        util_add_one(db, new_blog_article, BLOG_TABLE).await
    }

    async fn find_active_paginated(
        db: &Data<Database>,
        page: usize,
        page_size: usize,
    ) -> Option<Vec<BlogArticle>> {
        util_find_active_paginated(
            db,
            BLOG_TABLE,
            page,
            page_size,
            Some("published_at"),
            Some(true),
        )
        .await
    }

    // async fn update_one(db: &Data<Database>, blog_article: BlogArticle) -> Option<BlogArticle> {
    //     util_update_record(db, blog_article, BLOG_TABLE).await
    // }

    // async fn find_all_including_deleted(db: &Data<Database>) -> Option<Vec<BlogArticle>> {
    //     util_find_all(&db, BLOG_TABLE).await
    // }

    // async fn find_all_deleted(db: &Data<Database>) -> Option<Vec<BlogArticle>> {
    //     util_query_deleted(&db, BLOG_TABLE).await
    // }

    // async fn delete_one(db: &Data<Database>, id: String) -> Option<Vec<BlogArticle>> {
    //     util_remove_record(&db, id, BLOG_TABLE).await
    // }

    async fn search_content(db: &Data<Database>, search_term: String) -> Option<Vec<BlogArticle>> {
        let fields = vec![
            "summary".to_string(),
            "keywords".to_string(),
            "table_of_contents".to_string(),
            "author".to_string(),
            "title".to_string(),
            "content".to_string(),
        ];
        let search_fields = create_or_conditional(&search_term, fields);
        util_fulltext_search(db, BLOG_TABLE, &search_fields).await
    }

    async fn search_slug_id(db: &Data<Database>, search_term: String) -> Option<Vec<BlogArticle>> {
        let search_fields = format!("slug = '{search_term}'");
        util_fulltext_search(db, BLOG_TABLE, &search_fields).await
    }

    async fn find_random_articles(
        db: &Data<Database>,
        number_of_records: Option<usize>,
    ) -> Option<Vec<BlogArticle>> {
        util_find_random_articles(db, BLOG_TABLE, number_of_records).await
    }

    async fn get_number_of_articles(db: &Data<Database>) -> Option<u64> {
        let number_of_articles: Option<u64> = util_count_records(db, BLOG_TABLE).await;

        match number_of_articles {
            Some(count) => Some(count),
            None => None,
        }
    }
}
