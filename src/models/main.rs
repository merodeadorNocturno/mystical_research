use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Main {
    pub url: String,
    pub schema_markup: MainSchemaMarkup, // Group schema markup related fields
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainSchemaMarkup {
    pub site_name: String,
    pub site_description: String,
    pub main_image_url: Option<String>,
    pub search_term_string: Option<String>,
}

impl Main {
    pub fn new(url: String, site_name: String, site_description: String) -> Self {
        Main {
            url,
            schema_markup: MainSchemaMarkup {
                // Initialize schema_markup struct
                site_name,
                site_description,
                main_image_url: None,
                search_term_string: None,
            },
        }
    }
}
