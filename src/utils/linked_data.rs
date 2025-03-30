use crate::models::blog_model::BlogArticle;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogArticleLD {
    slug: String,
    title: String,
    summary: String,
    image_urls: String,
    published_at: surrealdb::Datetime,
    updated_at: surrealdb::Datetime,
    author: String,
}

pub fn linked_data_blog_article(article: &BlogArticle) -> BlogArticleLD {
    let article_ld = article.clone();

    BlogArticleLD {
        author: article_ld.author.unwrap(),
        image_urls: article_ld.image_urls.unwrap(),
        published_at: article_ld.published_at.unwrap(),
        slug: article_ld.slug.unwrap(),
        summary: article_ld.summary.unwrap(),
        title: article_ld.title.unwrap(),
        updated_at: article_ld.updated_at.unwrap(),
    }
}

// id: Option<Thing>,
// title: Option<String>,
// table_of_contents: Option<String>,
// summary: Option<String>,
// content: Option<String>,
// keywords: Option<String>,
// page_type: Option<PageType>,
// deleted: Option<bool>,
// author: Option<String>,
// image_urls: Option<String>,
// created_at: Option<surrealdb::Datetime>,
// updated_at: Option<surrealdb::Datetime>,
// published_at: Option<surrealdb::Datetime>,
// slug: Option<String>,
