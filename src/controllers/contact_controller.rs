use crate::db::{config_db::Database, contact_db::ContactDB};
use crate::models::{
    contact_model::ContactFormData, general_model::TitleError, mock::mock_contact_home_page_object,
};
use crate::utils::{
    env_utils::{set_env_urls, PageConfiguration},
    fs_utils::{read_hbs_template, register_templates},
};
use actix_web::HttpRequest;
use actix_web::{
    web::{get, post, Data, Form, ServiceConfig},
    HttpResponse,
};
use handlebars::{Handlebars, RenderError};
use log::{error, info};
use serde_json::json;
use std::collections::BTreeMap;
use std::path::Path;
use surrealdb::sql::Value;

async fn htmx_contact() -> Result<String, RenderError> {
    let mut handlebars = Handlebars::new();
    let PageConfiguration { template_path, .. } = set_env_urls();

    let this_path = Path::new(&template_path);

    register_templates(this_path, &mut handlebars);
    let contact_hbs = "contact/contact";

    let section_template = match read_hbs_template(contact_hbs) {
        Ok(contents) => contents,
        Err(err) => {
            error!(
                "Failed to render contents for contact page: {}",
                err.to_string()
            );
            TitleError::new(err.to_string()).error
        }
    };

    let section_template = handlebars.render_template(&section_template, &json!({}))?;
    Ok(section_template)
}

async fn contact_html() -> Result<String, RenderError> {
    let PageConfiguration { template_path, .. } = set_env_urls();

    let mut handlebars = Handlebars::new();
    let this_path = Path::new(&template_path);

    register_templates(this_path, &mut handlebars);
    let contact_hbs = "index/index";

    let contact_home_template = match read_hbs_template(contact_hbs) {
        Ok(contents) => contents,
        Err(err) => {
            error!(
                "Failed to render contents for contact page: {}",
                err.to_string()
            );
            TitleError::new(err.to_string()).error
        }
    };

    let section_template = match handlebars.render_template(
        &contact_home_template,
        &json!(&mock_contact_home_page_object()),
    ) {
        Ok(template) => template,
        Err(err) => {
            error!(
                "Failed to render contents for contact page: {}",
                err.to_string()
            );
            TitleError::new(err.to_string()).error
        }
    };
    Ok(section_template)
}

async fn post_htmx_contact(
    form: Form<ContactFormData>,
    db: Data<Database>,
) -> Result<String, RenderError> {
    let contact_data = form.into_inner();
    info!("Received contact form submission: {:?}", &contact_data);

    let mut records = BTreeMap::new();
    records.insert("email", Value::from(contact_data.email.to_string()));
    records.insert("message", Value::from(contact_data.message.to_string()));
    records.insert("name", Value::from(contact_data.name.to_string()));
    records.insert(
        "subject",
        Value::from(contact_data.subject.unwrap().to_string()),
    );

    let contact_form_data: ContactFormData = ContactFormData::builder()
        .name(records.get("name").unwrap().to_string())
        .email(records.get("email").unwrap().to_string())
        .subject(records.get("subject").unwrap().to_string())
        .message(records.get("message").unwrap().to_string())
        .deleted(false)
        .build();

    let add_contact_form_data: Option<ContactFormData> =
        Database::save_one(&db, contact_form_data).await;

    info!("Saved email {:?}", add_contact_form_data);

    let handlebars = Handlebars::new();
    let contact_hbs = "contact/contact_success_response";

    let contact_home_template = match read_hbs_template(contact_hbs) {
        Ok(contents) => contents,
        Err(err) => {
            error!(
                "Failed to render contents for contact page: {}",
                err.to_string()
            );
            TitleError::new(err.to_string()).error
        }
    };

    // --- Server-Side Validation ---

    let section_template = match handlebars.render_template(
        &contact_home_template,
        &json!(&mock_contact_home_page_object()),
    ) {
        Ok(template) => template,
        Err(err) => {
            error!(
                "Failed to render contents for contact page: {}",
                err.to_string()
            );
            TitleError::new(err.to_string()).error
        }
    };
    Ok(section_template)
}

pub fn contact_controller(cfg: &mut ServiceConfig) {
    cfg.route(
        "/htmx/contact",
        get().to(|| async move {
            let contact_template = htmx_contact().await;
            match contact_template {
                Ok(template) => HttpResponse::Ok()
                    .content_type("text/html")
                    .body(template),
                Err(err) => HttpResponse::InternalServerError()
                    .content_type("text/html")
                    .body(format!(
                        "<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load contact page: {}</span>",
                        err
                    )),
            }
        }),
    );

    cfg.route(
        "/contact.html",
        get().to(|| async move {
            let contact_home_template = contact_html().await;
            match contact_home_template {
                Ok(template) => HttpResponse::Ok()
                    .content_type("text/html")
                    .body(template),
                Err(err) => HttpResponse::InternalServerError()
                    .content_type("text/html")
                    .body(format!(
                        "<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load contact page: {}</span>",
                        err
                    )),
            }
        }),
    );

    cfg.route(
        "/contact",
        post().to(|_req: HttpRequest, form, db: Data<Database>| async move {
            let new_contact = post_htmx_contact(form, db).await;

            match new_contact {
                Ok(created_contact) => HttpResponse::Ok()
                    .content_type("text/html")
                    .body(created_contact),
                Err(e) => HttpResponse::Ok()
                    .content_type("text/html")
                    .append_header(("HX-Trigger", "error_contact_table"))
                    .body(
                        format!(
                            "<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load Enterprise: {}</span>",
                            e.to_string()
                        )
                    )
            }
        }),
    );
}
