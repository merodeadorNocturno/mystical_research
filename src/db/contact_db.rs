use crate::db::config_db::Database;
use crate::models::contact_model::ContactFormData;
use crate::utils::{crud_utils::*, general_utils::create_or_conditional};
use actix_web::web::Data;
use async_trait::async_trait;

const CONTACT_TABLE: &str = "contact";

#[async_trait]
pub trait ContactDB {
    async fn find_all(
        db: &Data<Database>,
        number_of_records: Option<usize>,
    ) -> Option<Vec<ContactFormData>>;
    async fn save_one(db: &Data<Database>, contact: ContactFormData) -> Option<ContactFormData>;
    async fn search_contact(
        db: &Data<Database>,
        search_term: String,
    ) -> Option<Vec<ContactFormData>>;
    async fn find_one(db: &Data<Database>, id: String) -> Option<ContactFormData>;
}

#[async_trait]
impl ContactDB for Database {
    async fn find_all(
        db: &Data<Database>,
        number_of_records: Option<usize>,
    ) -> Option<Vec<ContactFormData>> {
        util_find_active_records(db, CONTACT_TABLE, number_of_records).await
    }

    async fn save_one(db: &Data<Database>, contact: ContactFormData) -> Option<ContactFormData> {
        util_add_one(db, contact, CONTACT_TABLE).await
    }

    async fn search_contact(
        db: &Data<Database>,
        search_term: String,
    ) -> Option<Vec<ContactFormData>> {
        let fields = vec![
            "name".to_string(),
            "email".to_string(),
            "subject".to_string(),
            "message".to_string(),
        ];
        let search_fields = create_or_conditional(&search_term, fields);
        util_fulltext_search(db, CONTACT_TABLE, &search_fields).await
    }
    async fn find_one(db: &Data<Database>, id: String) -> Option<ContactFormData> {
        util_find_one(db, id, CONTACT_TABLE).await
    }
}
