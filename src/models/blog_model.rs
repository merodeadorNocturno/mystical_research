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
    pub blog_post_title: String,
    pub blog_post_headline: String,
    pub blog_post_description: String,
    pub blog_post_date_published: Option<DateTime<Local>>,
    pub blog_post_date_modified: Option<DateTime<Local>>,
    pub blog_post_image_urls: Option<Vec<BlogImage>>,
    pub blog_post_author_name: String,
    pub blog_post_logo_url: String,
    pub blog_post_logo_alt: String,
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
    pub blog_image_width: String,
    pub blog_image_height: String,
    pub blog_image_alt: String,
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
