use crate::models::ai::{AiRequest, GenerateContentResponse};
use log::{info, warn};
use reqwest::StatusCode;

pub async fn generate_content(ai_request: AiRequest) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut return_response = String::new();

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
            Ok(response_body) => {
                info!(
                    "{}",
                    format!(
                        "Parsed Response (GenerateContentResponse): {:?}",
                        response_body
                    )
                );

                if let Some(candidate) = response_body.candidates.first() {
                    if let Some(part) = candidate.content.parts.first() {
                        let text = &part.text;
                        return_response = text.to_string();
                        println!("First part: {}", text);
                    }
                }
            }
            Err(json_err) => {
                // Handle the error
                println!("JSON Parsin Error: {:?}", json_err);
            }
        }
    } else {
        warn!("{}", format!("API Error Response Text:\n{}", response_text));
        warn!(
            "{}",
            format!("API request failed with status: {:?}", response_status)
        );
    }

    Ok(return_response)
}
