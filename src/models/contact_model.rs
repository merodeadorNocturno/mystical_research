use crate::models::{general_model::PageType, index_model::HeaderData};
use serde::{Deserialize, Serialize};
use validator::Validate;

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

#[derive(Deserialize, Debug, Serialize, Validate)] // Added Serialize for potential logging/DB
pub struct ContactFormData {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    pub subject: Option<String>, // Subject might be optional
    #[validate(length(min = 10, message = "Message must be at least 10 characters"))]
    pub message: String,
    pub deleted: bool,
}

impl ContactFormData {
    pub fn builder() -> ContactFormDataBuilder {
        ContactFormDataBuilder::new()
    }
}

#[derive(Debug, Default)]
pub struct ContactFormDataBuilder {
    pub name: String,
    pub email: String,
    pub subject: Option<String>, // Subject might be optional
    pub message: String,
    pub deleted: bool,
}

impl ContactFormDataBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn email(mut self, email: String) -> Self {
        self.email = email;
        self
    }

    pub fn message(mut self, message: String) -> Self {
        self.message = message;
        self
    }

    pub fn subject(mut self, subject: String) -> Self {
        self.subject = Some(subject);
        self
    }

    pub fn deleted(mut self, deleted: bool) -> Self {
        self.deleted = deleted;
        self
    }

    pub fn build(self) -> ContactFormData {
        ContactFormData {
            name: self.name,
            email: self.email,
            subject: self.subject,
            message: self.message,
            deleted: self.deleted,
        }
    }
}
