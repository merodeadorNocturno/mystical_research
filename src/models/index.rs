use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
    pub url: String,
    pub schema_markup: IndexSchemaMarkup, // Group schema markup related fields
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexSchemaMarkup {
    pub site_name: String,
    pub site_description: String,
    pub main_image_url: Option<String>,
    pub search_term_string: Option<String>,
    pub canonical_url: Option<String>,
}

impl Index {
    pub fn new(url: String, site_name: String, site_description: String) -> Self {
        Index {
            url,
            schema_markup: IndexSchemaMarkup {
                // Initialize schema_markup struct
                site_name,
                site_description,
                main_image_url: None,
                search_term_string: None,
                canonical_url: None,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexBody {
    pub title: String,
    pub description: String,
    pub explore_url: String,
    pub learn_more_url: String,
    pub explore_label: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexFeaturedSection {
    pub section_title: String,
    pub section_description: String,
    pub section_content: String,
    pub section_image_url: Option<String>,
    pub section_alt_text: Option<String>,
    pub section_link_url: Option<String>,
    pub section_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexPage {
    pub body: IndexBody,
    pub schema_markup: IndexSchemaMarkup,
    pub featured: Vec<IndexFeaturedSection>,
    pub header: HeaderData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NavLink {
    pub label: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderData {
    pub site_name: String,
    pub site_description: String,
    pub navigation_links: Vec<NavLink>,
    pub logo_url: Option<String>, // Optional logo URL
}
