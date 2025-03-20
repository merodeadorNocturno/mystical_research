use crate::models::{
    ai_model::{AiResponse, GenerateContentResponse},
    mock::mock_blog_home_page_object,
};
use crate::utils::{
    ai_utils::{create_ai_request, generate_content},
    env_utils::{set_env_urls, PageConfiguration},
    fs_utils::{read_hbs_template, register_templates},
    response_utils::create_blog_structure_from_response,
};
use actix_web::{
    web::{get, ServiceConfig},
    HttpResponse,
};
use handlebars::{Handlebars, RenderError};
use log::error;
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

async fn load_blog_index() -> Result<String, RenderError> {
    let mut handlebars = Handlebars::new();
    let PageConfiguration { template_path, .. } = set_env_urls();

    let this_path = Path::new(&template_path);

    register_templates(this_path, &mut handlebars);
    let blog_home_hbs = "blog/blog_home";

    let section_template = match read_hbs_template(&blog_home_hbs) {
        Ok(contents) => contents,
        Err(err) => {
            error!(
                "Failed to render contents for blog home page: {}",
                err.to_string()
            );
            TitleError::new(err.to_string()).error
        }
    };

    let section_template = handlebars.render_template(&section_template, &json!({}))?;
    Ok(section_template)
}

async fn load_blog_article() -> Result<String, RenderError> {
    let mut handlebars = Handlebars::new();
    let PageConfiguration { template_path, .. } = set_env_urls();

    let this_path = Path::new(&template_path);

    let mut ai_response: GenerateContentResponse = GenerateContentResponse::new(vec![]);
    let mut _ai_error: String = String::new();

    register_templates(this_path, &mut handlebars);
    let blog_article_hbs = "blog/blog_article_og";

    let section_template = match read_hbs_template(&blog_article_hbs) {
        Ok(contents) => contents,
        Err(err) => {
            error!(
                "Failed to render contents for blog article page: {}",
                err.to_string()
            );
            TitleError::new(err.to_string()).error
        }
    };

    // let token_string =
    //     "Pretend you have a PhD in ethics. Write a blog article on the ethics and contradictions of theology and religious practices.
    //     Use the first paragraph for the title.
    // Use the second paragraph for table of contents, separated by commas.
    // Use the third paragraph for an abstract summary.
    // Use the last paragraph for a comma separated list of keywords.";
    let token_string = "Pretend you have a PhD in philology and a master's degree in anthropology.
      Write an 6 paragraph essay titled
      'The Sensual Language of Moonlight: Dreams, Intuition, and the Mystical Unveiling.'
      Focus on the exploration of dreams, inner experience and emphasis on sensuality and mystical traditions,
      this article should examine how the sensory, sexual, and aesthetic qualities of moonlight are culturally linked to dreams,
      intuition, and mystical insights. It would explore anthropological accounts of how different societies interpret moonlit nights as conducive to altered states of consciousness, sensual awakenings, or encounters with the mystical.
      The article could consider how the symbolic imagery associated with the moon in dreams is understood as a form of mystical communication.
      Avoid making statements that are not supported by evidence or research. e.g. 'The cool touch of moonlight on the skin' is not acceptable.
      Use the first paragraph for the title.
      Use the second paragraph for table of contents, separated by commas.
      Use the third paragraph for an abstract summary.
      Use the last paragraph for a comma separated list of keywords.";

    let ai_request = create_ai_request(token_string);

    match generate_content(ai_request).await {
        Ok(AiResponse {
            response_body: rb, ..
        }) => {
            ai_response = rb;
        }
        Err(err) => {
            error!(
                "Failed to generate content for blog article: {}",
                err.to_string()
            );
            _ai_error = format!("Failed to generate article content: {}", err.to_string());
        }
    };

    let ai_response_from_google = &ai_response.candidates[0].content.parts[0].text;

    let blog_structure = create_blog_structure_from_response(ai_response_from_google);

    let section_template = handlebars.render_template(&section_template, &json!(blog_structure))?;

    Ok(section_template)
}

async fn load_blog_html() -> Result<String, RenderError> {
    let PageConfiguration { template_path, .. } = set_env_urls();

    let mut handlebars = Handlebars::new();
    let this_path = Path::new(&template_path);

    register_templates(this_path, &mut handlebars);
    let blog_home_hbs = "index/index";

    let blog_home_template = match read_hbs_template(&blog_home_hbs) {
        Ok(template) => template,
        Err(err) => {
            eprintln!("Failed to read blog home template: {}", err);
            String::new()
        }
    };

    let section_blog_home_template = match handlebars
        .render_template(&blog_home_template, &json!(&mock_blog_home_page_object()))
    {
        Ok(template) => template,
        Err(err) => {
            eprintln!("Failed to render blog home template: {}", err);
            String::new()
        }
    };

    Ok(section_blog_home_template)
}

pub fn blog_home_html(cfg: &mut ServiceConfig) {
    cfg.route(
        "/blog",
        get().to(|| async move {
            let blog_home_template = load_blog_index().await;
            match blog_home_template {
                Ok(template) => HttpResponse::Ok()
                    .content_type("text/html")
                    .body(template),
                Err(err) => HttpResponse::InternalServerError()
                    .content_type("text/html")
                    .body(format!(
                        "<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load blog home page: {}</span>",
                        err.to_string()
                    )),
            }
        }),
    );

    cfg.route(
            "/blog/article",
            get().to(|| async move {
                let blog_article_template = load_blog_article().await;
                match blog_article_template {
                    Ok(template) => HttpResponse::Ok()
                        .content_type("text/html")
                        .body(template),
                    Err(err) => HttpResponse::InternalServerError()
                        .content_type("text/html")
                        .body(format!(
                            "<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load blog article page: {}</span>",
                            err.to_string()
                        )),
                }
            }),
        );

    cfg.route(
      "/blog_home.html",
      get().to(|| async move {
        let blog_home_template = load_blog_html().await;
        match blog_home_template {
            Ok(template) => HttpResponse::Ok()
                .content_type("text/html")
                .body(template),
            Err(err) => HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(format!(
                    "<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load blog home page: {}</span>",
                    err.to_string()
                )),
        }
      })
    );
}
