use crate::db::config_db::Database;
use crate::models::mailing_model::MailingList;
use crate::utils::crud_utils::*;
use actix_web::web::Data;
use async_trait::async_trait;

const MAILING_TABLE: &str = "mailing_list";

#[async_trait]
pub trait MailingListDB {
    async fn save_one(db: &Data<Database>, mailing_list: MailingList) -> Option<MailingList>;
    // async fn find_all(
    //     db: &Data<Database>,
    //     number_of_records: Option<usize>,
    // ) -> Option<Vec<MailingList>>;
    // async fn update_mailing_list(
    //     db: &Database,
    //     id: i32,
    //     mailing_list: &MailingList,
    // ) -> Result<(), String>;
    // async fn delete_email(db: &Data<Database>, mailing_list: MailingList) -> Option<MailingList>;
}

#[async_trait]
impl MailingListDB for Database {
    async fn save_one(db: &Data<Database>, mailing_list: MailingList) -> Option<MailingList> {
        util_add_one(db, mailing_list, MAILING_TABLE).await
    }

    // async fn find_all(
    //     db: &Data<Database>,
    //     number_of_records: Option<usize>,
    // ) -> Option<Vec<MailingList>> {
    //     util_find_active_records(db, MAILING_TABLE, number_of_records).await
    // }

    // async fn delete_email(db: &Data<Database>, mailing_list: MailingList) -> Option<MailingList> {
    //     util_update_record(db, mailing_list, MAILING_TABLE).await
    // }
}
