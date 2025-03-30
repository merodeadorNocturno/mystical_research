use crate::models::{general_model::PageType, index_model::HeaderData};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TopicHomePage {
    pub header: HeaderData,
    pub linked_data: TopicHomeSchemaMarkup,
    pub categories: Vec<TopicCategory>,
    pub section: PageType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TopicHomeSchemaMarkup {
    pub topic_page_title: String,
    pub site_description: String,
    pub canonical_url: String,
    pub site_name: String,
    pub site_logo_url: String,
    pub search_action_target: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TopicCategory {
    pub category_icon: String,
    pub category_title: String,
    pub category_description: String,
    pub category_url: String,
}
