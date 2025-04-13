use crate::db::config_db::Database;
use crate::models::{contact_models::ContactFormData, mock::mock_contact_home_page_object};
use crate::utils::{
    env_utils::{set_env_urls, PageConfiguration},
    fs_utils::{read_hbs_template, register_templates},
};
use actix_web::{
    web::{get, post, Data, Form, ServiceConfig},
    HttpResponse, Responder,
};
use handlebars::{Handlebars, RenderError};
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
struct TitleError {
    pub error: String,
}

impl TitleError {
    pub fn new(error: String) -> TitleError {
        TitleError { error }
    }
}

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

// async fn post_contact(form: Form<ContactFormData>) -> Result<String, RenderError> {
//     let contact_data = form.into_inner();
//     info!("Received contact form submission: {:?}", contact_data);

//     // --- Server-Side Validation ---
//     let mut errors: Vec<String> = Vec::new();
//     if contact_data.name.trim().is_empty() {
//         errors.push("Name cannot be empty.".to_string());
//     }
//     if contact_data.email.trim().is_empty() || !contact_data.email.contains('@') {
//         // Basic email check
//         errors.push("Please provide a valid email address.".to_string());
//     }
//     if contact_data.message.trim().len() < 5 {
//         // Basic length check
//         errors.push("Message must be at least 5 characters long.".to_string());
//     }

//     // If using validator crate:
//     // if let Err(validation_errors) = contact_data.validate() {
//     //     // Format validation_errors into user-friendly messages
//     //     // For simplicity, just joining them here
//     //     let error_messages = validation_errors.field_errors().iter().flat_map(|(_, errors)| errors.iter().filter_map(|e| e.message.as_ref().map(|m| m.to_string()))).collect::<Vec<_>>().join(" ");
//     //     return HttpResponse::BadRequest().content_type("text/html").body(format!(
//     //         "<div class=\"form-response error\">Validation failed: {}</div>", error_messages
//     //     ));
//     // }

//     if !errors.is_empty() {
//         let error_html = errors
//             .iter()
//             .map(|e| format!("<p>{}</p>", e))
//             .collect::<String>();
//         // Return 400 Bad Request with error messages for HTMX target
//         return HttpResponse::BadRequest()
//             .content_type("text/html")
//             .body(format!(
//                 "<div class=\"form-response error\">Please correct the following errors:{}</div>",
//                 error_html
//             ));
//     }

//     // --- Process the data (e.g., save to DB, send email) ---
//     // Placeholder: Simulate processing
//     info!("Processing contact data for: {}", contact_data.email);
//     // Replace this with your actual logic (e.g., database insert)
//     let save_result: Result<(), String> = Ok(()); // Simulate success

//     match save_result {
//         Ok(_) => {
//             info!(
//                 "Successfully processed contact form from {}",
//                 contact_data.email
//             );
//             // Return success message for HTMX target
//             // Optionally clear the form by returning an empty form or keep fields filled
//             HttpResponse::Ok().content_type("text/html").body(
//                  "<div class=\"form-response success\">Thank you for your message! We will get back to you soon.</div>"
//                  // To clear the form, you could return the form structure again, empty:
//                  // "<form id=\"contact-form\" ...> ... <button>Send</button></form><div id=\"contact-form-response\">...success msg...</div>"
//                  // Or use hx-swap="outerHTML" on the form and return the success message *plus* a new empty form.
//              )
//         }
//         Err(e) => {
//             error!("Failed to process contact form: {}", e);
//             // Return server error message for HTMX target
//             HttpResponse::InternalServerError().content_type("text/html").body(
//                   "<div class=\"form-response error\">Sorry, there was an error processing your request. Please try again later.</div>"
//              )
//         }
//     }
// }

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

    // cfg.route(
    //     "/contact",
    //     post().to(|form, db:Data<Database>| async move {
    //         let contact_home_template = post_contact(form).await;
    //         match contact_home_template {
    //             Ok(template) => HttpResponse::Ok()
    //                 .content_type("text/html")
    //                 .body(template),
    //             Err(err) => HttpResponse::InternalServerError()
    //                 .content_type("text/html")
    //                 .body(format!(
    //                     "<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load contact page: {}</span>",
    //                     err
    //                 )),
    //         }
    //     }),
    // );
}
