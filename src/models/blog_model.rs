use crate::models::ai_model::BlogStructure;
use crate::models::index_model::HeaderData;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use super::general_model::PageType;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlogHomePage {
    pub body: BlogBody,
    pub schema_markup: BlogHomeSchemaMarkup,
    pub featured: Vec<BlogFeaturedSection>,
    pub header: HeaderData,
    pub section: PageType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlogPostSchemaMarkup {
    pub title: String,
    pub headline: String,
    pub description: String,
    pub content: String,
    pub date_published: Option<DateTime<Local>>,
    pub date_modified: Option<DateTime<Local>>,
    pub image_urls: Option<Vec<BlogImage>>,
    pub author_name: String,
    pub logo_url: String,
    pub logo_alt: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlogHomeSchemaMarkup {
    pub blog_title: String,
    pub blog_description: String,
    pub blog_name: String,
    pub blog_url: String,
    pub blog_logo_url: String,
    pub blog_search_action: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlogImage {
    pub blog_image_url: String,
    pub blog_image_alt: Option<String>,
    pub blog_image_width: Option<String>,
    pub blog_image_height: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlogBody {
    pub structure: BlogStructure,
    pub author: String,
    pub image_urls: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlogFeaturedSection {
    pub blog_id: String,
    pub blog_title: String,
    pub blog_description: String,
    pub blog_image_urls: Option<Vec<BlogImage>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BlogArticle {
    // pub id: Option<Thing>,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub keywords: Option<String>,
    pub content: Option<String>,
    pub table_of_contents: Option<String>,
    pub page_type: Option<PageType>,
    pub deleted: Option<bool>,
    pub author: Option<String>,
    pub image_urls: Option<String>,
    pub created_at: Option<surrealdb::Datetime>,
    pub updated_at: Option<surrealdb::Datetime>,
    pub published_at: Option<surrealdb::Datetime>,
}

impl BlogArticle {
    pub fn builder() -> BlogArticleBuilder {
        BlogArticleBuilder::new()
    }
}

#[derive(Debug, Default)]
pub struct BlogArticleBuilder {
    // Struct 2: BlogArticleBuilder
    // id: Option<Thing>,
    title: Option<String>,
    summary: Option<String>,
    keywords: Option<String>,
    content: Option<String>,
    table_of_contents: Option<String>,
    page_type: Option<PageType>,
    deleted: Option<bool>,
    author: Option<String>,
    image_urls: Option<String>,
    created_at: Option<surrealdb::Datetime>,
    updated_at: Option<surrealdb::Datetime>,
    published_at: Option<surrealdb::Datetime>,
}

impl BlogArticleBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }
    pub fn author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }
    pub fn image_urls(mut self, image_urls: String) -> Self {
        self.image_urls = Some(image_urls);
        self
    }
    pub fn summary(mut self, summary: String) -> Self {
        self.summary = Some(summary);
        self
    }
    pub fn keywords(mut self, keywords: String) -> Self {
        self.keywords = Some(keywords);
        self
    }
    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }
    pub fn created_at(mut self, created_at: &surrealdb::Datetime) -> Self {
        self.created_at = Some(created_at.clone());
        self
    }
    pub fn updated_at(mut self, updated_at: &surrealdb::Datetime) -> Self {
        self.updated_at = Some(updated_at.clone());
        self
    }
    pub fn published_at(mut self, published_at: &surrealdb::Datetime) -> Self {
        self.published_at = Some(published_at.clone());
        self
    }
    pub fn table_of_contents(mut self, table_of_contents: String) -> Self {
        self.table_of_contents = Some(table_of_contents);
        self
    }
    pub fn page_type(mut self, page_type: PageType) -> Self {
        self.page_type = Some(page_type);
        self
    }
    pub fn deleted(mut self, deleted: bool) -> Self {
        self.deleted = Some(deleted);
        self
    }

    pub fn build(self) -> BlogArticle {
        BlogArticle {
            // id: Some(self.id.expect("id must be set")),
            author: Some(self.author.expect("header must be set")),
            title: Some(self.title.expect("title must be set")),
            image_urls: Some(self.image_urls.expect("body must be set")),
            content: Some(self.content.expect("content must be set")),
            summary: Some(self.summary.expect("summary must be set")),
            keywords: Some(self.keywords.expect("keywords must be set")),
            table_of_contents: Some(
                self.table_of_contents
                    .expect("table_of_contents must be set"),
            ),
            page_type: Some(self.page_type.expect("page_type must be set")),
            deleted: Some(self.deleted.expect("deleted must be set")),
            created_at: Some(self.created_at.expect("created_at must be set")),
            updated_at: Some(self.updated_at.expect("updated_at must be set")),
            published_at: Some(self.published_at.expect("updated_at must be set")),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlogArticleBody {
    pub structure: BlogStructure,
    pub author: String,
    pub image_urls: Option<Vec<String>>,
    pub content_sections: Vec<BlogSection>, // Added content sections to map to template structure
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlogSection {
    pub section_id: String,      // e.g., "history", "cards", "spreads"
    pub section_title: String,   // e.g., "History", "Cards", "Keywords"
    pub section_content: String, // HTML content for the section, could be used for <p>{{{content}}}</p>
    pub section_image_gallery: Option<Vec<BlogImage>>, // For image galleries in sections
    pub section_card_grid: Option<Vec<BlogCard>>, // For card grids like in "cards" section
    pub section_spread_example: Option<BlogSpreadExample>, // For spread example like in "spreads" section
                                                           // Add other section specific fields as needed based on template
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlogCard {
    pub card_image: BlogImage,
    pub card_text: String, // e.g., "Major Arcana", "Minor Arcana"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlogSpreadExample {
    pub spread_image: BlogImage,
    pub spread_text: String, // e.g., "The Celtic Cross is one of the most popular tarot spreads."
}
