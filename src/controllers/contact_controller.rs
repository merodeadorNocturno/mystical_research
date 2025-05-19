use crate::db::{config_db::Database, contact_db::ContactDB};
use crate::models::{
    contact_model::ContactFormData, general_model::TitleError, mock::mock_contact_home_page_object,
};
use crate::utils::{
    env_utils::{set_env_urls, PageConfiguration},
    fs_utils::{read_hbs_template, register_templates},
};
use actix_csrf::extractor::CsrfToken;
use actix_web::HttpRequest; // HttpRequest might still be needed for the POST route
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

async fn htmx_contact(token: CsrfToken) -> Result<String, RenderError> {
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

    // Extract CSRF token value using the CsrfToken extractor
    let csrf_token_value = token.get().to_string();


    let section_template = handlebars.render_template(&section_template, &json!({ "csrf_token": csrf_token_value }))?;
    Ok(section_template)
}

async fn contact_html(token: CsrfToken) -> Result<String, RenderError> {
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
    
    // Extract CSRF token value using the CsrfToken extractor
    let csrf_token_value = token.get().to_string();

    // Add csrf_token to the mock object or create a new context
    let contact_page_data = mock_contact_home_page_object();
    // This assumes mock_contact_home_page_object() returns a serde_json::Value or a struct that can be merged/extended.
    // If it's a struct, you might need to create a new struct that includes the csrf_token or use a map.
    // For simplicity, let's assume we can convert it to a map and insert the token.
    let mut data_map = match serde_json::to_value(&contact_page_data) {
        Ok(serde_json::Value::Object(map)) => map,
        _ => serde_json::Map::new(), // Or handle error
    };
    data_map.insert("csrf_token".to_string(), json!(csrf_token_value));


    let section_template = match handlebars.render_template(
        &contact_home_template,
        &data_map, // Pass the map with the token
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
        Value::from(contact_data.subject.unwrap_or_default().to_string()), // Handle Option
    );

    let contact_form_data: ContactFormData = ContactFormData::builder()
        .name(records.get("name").map(|v| v.to_string().replace("\"", "")).unwrap_or_default()) // Sanitize if necessary
        .email(records.get("email").map(|v| v.to_string().replace("\"", "")).unwrap_or_default())
        .subject(records.get("subject").map(|v| v.to_string().replace("\"", "")).unwrap_or_default())
        .message(records.get("message").map(|v| v.to_string().replace("\"", "")).unwrap_or_default())
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
        get().to(|token: CsrfToken| async move {
            match htmx_contact(token).await {
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
        get().to(|token: CsrfToken| async move {
            match contact_html(token).await {
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
