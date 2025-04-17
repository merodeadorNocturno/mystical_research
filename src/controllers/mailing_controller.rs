use crate::db::{config_db::Database, mailing_db::MailingListDB};
use crate::models::{general_model::TitleError, mailing_model::MailingList};
use crate::utils::fs_utils::read_hbs_template;
use actix_web::{
    web::{post, Data, Form, ServiceConfig},
    HttpRequest, HttpResponse,
};
use handlebars::{Handlebars, RenderError};
use log::{error, info};
use serde_json::json;
use std::collections::BTreeMap;
use surrealdb::sql::Value;

async fn post_htmx_mailing_list(
    form: Form<MailingList>,
    db: Data<Database>,
) -> Result<String, RenderError> {
    let email_data = form.into_inner();

    let mut my_email = BTreeMap::new();
    my_email.insert("email", Value::from(email_data.email.to_string()));

    let mailing_list: MailingList = MailingList::builder()
        .email(my_email.get("email").unwrap().to_string())
        .build();

    let add_mailing_list: Option<MailingList> = Database::save_one(&db, mailing_list).await;

    info!("Email added successfully: {:?}", &add_mailing_list);

    let handlebars = Handlebars::new();
    let mailing_hbs = "partials/newsletter_section";

    let mailing_list_template = match read_hbs_template(mailing_hbs) {
        Ok(contents) => contents,
        Err(err) => {
            error!("Error reading template: {:?}", err.to_string());
            TitleError::new(err.to_string()).error
        }
    };

    let section_template = match handlebars.render_template(&mailing_list_template, &json!({})) {
        Ok(template) => template,
        Err(err) => {
            error!("Error rendering template: {:?}", err.to_string());
            TitleError::new(err.to_string()).error
        }
    };

    Ok(section_template)
}

pub fn mailing_list_controller(cfg: &mut ServiceConfig) {
    cfg.route(
        "/htmx/mailing_list",
        post().to(|_req: HttpRequest, form, db: Data<Database>| async move {
            let new_mailing_list = post_htmx_mailing_list(form, db).await;
            match new_mailing_list {
                Ok(mailing_list) => HttpResponse::Ok()
                    .content_type("text/html")
                    .body(mailing_list),
                Err(err) => HttpResponse::Ok()
                    .content_type("text/html")
                    .body(
                        format!(
                            "<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load Enterprise: {}</span>",
                            err.to_string()
                        )
                    ),
            }
        }),
    );
}
