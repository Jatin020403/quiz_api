use std::env::{self};

use crate::model::{
    Content, GenerateContentRequest,
    GenerateContentResponse, GenerationConfig, Part
};

use gcp_auth::AuthenticationManager;

static MODEL_NAME: &str = "gemini-pro";

pub async fn generate_ai_content(input_str: String) -> Result<GenerateContentResponse, String> {
    let api_endpoint: String = match env::var("API_ENDPOINT") {
        Ok(s) => s,
        Err(err) => return Err(err.to_string()),
    };
    let project_id: String = match env::var("PROJECT_ID") {
        Ok(s) => s,
        Err(err) => return Err(err.to_string()),
    };
    let location_id: String = match env::var("LOCATION_ID") {
        Ok(s) => s,
        Err(err) => return Err(err.to_string()),
    }; // Sometimes called "region" in gCloud docs.

    println!("Endpoint : {:?}", api_endpoint);

    let endpoint_url = format!(
        "https://{api_endpoint}/v1beta1/projects/{project_id}/locations/{location_id}/publishers/google/models/{MODEL_NAME}:generateContent"
    );

    let authentication_manager: AuthenticationManager = match AuthenticationManager::new().await {
        Ok(s) => s,
        Err(err) => return Err(err.to_string()),
    };

    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let token = match authentication_manager.get_token(scopes).await {
        Ok(s) => s,
        Err(err) => return Err(err.to_string()),
    };

    let prompt = input_str;

    let payload = GenerateContentRequest {
        contents: vec![Content {
            role: "user".to_string(),
            parts: vec![Part::Text(prompt.to_string())],
        }],
        generation_config: Some(GenerationConfig {
            max_output_tokens: Some(2048),
            temperature: Some(0.4),
            top_p: Some(1.0),
            top_k: Some(32),
            ..Default::default()
        }),
        tools: None,
    };

    let resp: reqwest::Response = match reqwest::Client::new()
        .post(&endpoint_url)
        .bearer_auth(token.as_str())
        .json(&payload)
        .send()
        .await
    {
        Ok(s) => s,
        Err(err) => return Err(err.to_string()),
    };

    let gen_response: GenerateContentResponse = match resp.json::<GenerateContentResponse>().await {
        Ok(s) => s,
        Err(err) => return Err(err.to_string()),
    };

    Ok(gen_response)
}
