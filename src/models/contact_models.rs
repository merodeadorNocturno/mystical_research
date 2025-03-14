use crate::models::{general_model::PageType, index_model::HeaderData};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContactHomePage {
    pub header: HeaderData,
    pub section: PageType,
    pub hero_title: String,
    pub hero_description: String,
    pub contact_email: String,
    pub contact_phone: String,
    pub contact_address: String,
    pub schema_markup: ContactHomeSchemaMarkup,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContactHomeSchemaMarkup {
    pub contact_page_title: String,
    pub site_description: String,
    pub canonical_url: String,
    pub site_name: String,
    pub site_logo_url: String,
    pub search_action_target: String, // For search action URL template
}
