use crate::db::{config_db::Database, contact_db::ContactDB};
use crate::models::{
    contact_model::ContactFormData,
    general_model::{PageType, SearchQuery},
};
use actix_web::{
    error::ErrorNotFound,
    get,
    http::StatusCode,
    web::{Data, Path, Query, ServiceConfig},
    HttpResponse,
};
use log::error;

#[get("/contacts/search")]
#[tracing::instrument(name = "Search Contacts", skip(db))]
async fn contact_search(
    db: Data<Database>,
    query: Query<SearchQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let this_contacts = match &query.q {
        Some(search_term) => Database::search_contact(&db, search_term.to_string()).await,
        None => None,
    };

    match this_contacts {
        Some(contacts_found) => Ok(HttpResponse::Ok()
            .status(StatusCode::OK)
            .json(contacts_found)),
        None => Ok(HttpResponse::NotFound()
            .status(StatusCode::NOT_FOUND)
            .json("No contacts found")),
    }
}

#[get("/contacts")]
#[tracing::instrument(name = "Get Contacts", skip(db))]
async fn contacts(
    db: Data<Database>,
    query: Query<SearchQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let contacts = match &query.r {
        Some(number_of_records) => {
            Database::find_all(&db, Some(number_of_records.clone() as usize)).await
        }
        None => None,
    };

    match contacts {
        Some(contacts_found) => Ok(HttpResponse::Ok()
            .status(StatusCode::OK)
            .json(contacts_found)),
        None => {
            error!("No contacts found");
            Err(ErrorNotFound("No contacts found"))
        }
    }
}

#[get("/contacts/{email}")]
#[tracing::instrument(
    name = "Get an email address"
    skip(db),
    fields(thing_id = %id)
)]
async fn contacts_email(
    db: Data<Database>,
    id: Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let contact = Database::find_one(&db, id.to_string()).await;

    match contact {
        Some(contact_found) => Ok(HttpResponse::Ok()
            .status(StatusCode::OK)
            .json(contact_found)),
        None => {
            error!("No contact found");
            Err(ErrorNotFound("No contact found"))
        }
    }
}

// #[put("/contacts/{id}")]
// #[tracing::instrument(
//     name = "set a contact",
//     skip(db),
//     fields(
//         id = body.id,
//         name = %body.name,
//         email = %body.email,
//         subject = body.subject,
//         message = body.message,
//         deleted = body.deleted,
//     )
// )]
// async fn put_contacts_id(db: Data<Database>, body: Json<>) -> {
//     let is_valid = body.validate();

// }

pub fn contact_api_controller(cfg: &mut ServiceConfig) {
    cfg.service(contact_search);
    cfg.service(contacts);
}
