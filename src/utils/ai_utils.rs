use crate::models::ai_model::{
    AiRequest, AiResponse, Content, GenerateContentRequest, GenerateContentResponse, Part,
};
use crate::utils::env_utils::create_ai_request_string;
use log::{debug, warn};
use reqwest::StatusCode;

pub fn create_ai_request(request_text: &str) -> AiRequest {
    let api_endpoint = create_ai_request_string();
    debug!("{:?}", api_endpoint);

    let request_body = GenerateContentRequest {
        contents: vec![Content {
            parts: vec![Part {
                text: request_text.to_string(),
            }],
        }],
    };

    AiRequest {
        api_endpoint: api_endpoint.to_string(),
        request_body,
    }
}

pub async fn generate_content(
    ai_request: AiRequest,
) -> Result<AiResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut server_response_string = String::new();
    let mut server_response_json = GenerateContentResponse::new(vec![]);

    let AiRequest {
        api_endpoint,
        request_body,
    } = ai_request;

    let response = client
        .post(api_endpoint)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    let response_status: &StatusCode = &response.status().clone();
    let response_bytes = response.bytes().await?;

    let response_text: String = String::from_utf8(response_bytes.to_vec())?;
    let response_json: Result<GenerateContentResponse, _> = serde_json::from_slice(&response_bytes);

    if response_status.is_success() {
        match response_json {
            Ok(this_response_body) => {
                server_response_json = this_response_body.clone();

                if let Some(candidate) = this_response_body.candidates.first() {
                    if let Some(part) = candidate.content.parts.first() {
                        let text = &part.text;
                        server_response_string = text.to_string();
                    }
                }
            }
            Err(json_err) => {
                // Handle the error
                warn!("JSON Parsing Error: {:?}", json_err);
            }
        }
    } else {
        warn!("{}", format!("API Error Response Text:\n{}", response_text));
        warn!(
            "{}",
            format!("API request failed with status: {:?}", response_status)
        );
    }

    Ok(AiResponse {
        response_body: server_response_json,
        response_string: server_response_string,
    })
}
