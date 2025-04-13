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
    pub linked_data: ContactHomeSchemaMarkup,
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

#[derive(Deserialize, Debug, Serialize)] // Added Serialize for potential logging/DB
                                         // #[derive(Deserialize, Debug, Serialize, Validate)] // If using validator crate
pub struct ContactFormData {
    // #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,
    // #[validate(email(message = "Invalid email format"))]
    pub email: String,
    pub subject: Option<String>, // Subject might be optional
    // #[validate(length(min = 10, message = "Message must be at least 10 characters"))]
    pub message: String,
}
