use crate::models::{general_model::PageType, index_model::HeaderData};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AboutHomePage {
    pub body: AboutBody,
    pub linked_data: AboutHomeSchemaMarkup,
    pub header: HeaderData,
    pub section: PageType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AboutBody {
    pub structure: AboutStructure,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AboutStructure {
    pub title: String,
    pub description: String,
    pub about_section_title: String, // Renamed "about" to avoid keyword clash and be more descriptive
    pub items: Vec<AboutItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AboutItem {
    pub title: String,
    pub description: String,
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AboutHomeSchemaMarkup {
    pub about_page_title: String,
    pub site_description: String,
    pub canonical_url: String,
    pub site_name: String,
    pub site_logo_url: String,
    pub search_action_target: String,
}
