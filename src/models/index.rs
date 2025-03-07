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
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexBody {}
