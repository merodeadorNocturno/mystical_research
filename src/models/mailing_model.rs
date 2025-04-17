use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct MailingList {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

impl MailingList {
    pub fn builder() -> MailingListBuilder {
        MailingListBuilder::new()
    }
}

#[derive(Debug, Default)]
pub struct MailingListBuilder {
    pub email: String,
}

impl MailingListBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn email(mut self, email: String) -> Self {
        self.email = email;
        self
    }

    pub fn build(self) -> MailingList {
        MailingList { email: self.email }
    }
}
