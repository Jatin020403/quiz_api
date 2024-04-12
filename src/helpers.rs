use std::env::{self};

use crate::initialiser::Argon;

use crate::model::{
    Content, GenerateContentRequest, GenerateContentResponse, GenerationConfig, Part,
};
use bson::{doc, document, to_document};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use gcp_auth::AuthenticationManager;

extern crate mongodb;
// use chrono::prelude::*;
use mongodb::{bson::Document, Database};
use uuid::Uuid;

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

pub fn hasher(password: String, argon: Argon) -> Result<String, String> {
    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = match argon.argon.hash_password(password.as_bytes(), &argon.salt) {
        Ok(s) => s.to_string(),
        Err(err) => {
            return Err(err.to_string());
        }
    };

    Ok(password_hash)
}

pub fn verify(inp_password: String, user_password: String, argon: Argon) -> Result<bool, String> {
    println!("{:#?}", inp_password);
    // Hash password to PHC string ($argon2id$v=19$...)
    let parsed_hash = match PasswordHash::new(&user_password) {
        Ok(s) => s,
        Err(err) => {
            return Err(err.to_string());
        }
    };

    Ok(Argon2::default()
        .verify_password(inp_password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub async fn get_user_pwd(
    username: String,
    coll: mongodb::Collection<Document>,
) -> Result<String, String> {
    let res = match coll.find_one(doc! { "username": username }, None).await {
        Ok(s) => s,
        Err(err) => return Err(err.to_string()),
    };

    // println!("{:#?}", res);

    match res {
        Some(ref document) => match document.get_str("password") {
            Ok(s) => Ok(s.to_string()),
            Err(err) => Err(err.to_string()),
        },
        None => Err("Password not found".to_string()),
    }
}

pub async fn make_flashcards(topic: String, count: i8) -> Result<String, String> {
    let prompt = format!(
        "Extract {:?} key points from the text. Present the information in a JSON format with two fields:
* key_points_array: An array containing each key point as a string.
* number_of_key_points: The number of elements in the key_points_array.
Use only these valid fields.

Text: {:?}

**Example:**
{{
    \"key_points_array\": [
        \"key point 0\",
        \"key point 1\",
        \"key point 2\",
        \"key point 3\",
        \"key point 4\",
    ],
    \"number_of_key_points\": 5
}}
",
        count, topic
    );

    let gen_response: GenerateContentResponse = match generate_ai_content(prompt).await {
        Ok(s) => s,
        Err(error) => return Err(error),
    };

    let part = &gen_response.candidates[0].content.parts[0];

    match part {
        Part::Text(t) => return Ok(t.to_string()),
        _ => return Err("not the same type".to_string()),
    };
}

pub async fn make_quiz(topic: String, count: i8) -> Result<String, String> {
    let prompt = format!("**Prompt:**

Given a passage of text `{:?}` and an integer {:?}, generate a JSON object containing {:?} multiple choice questions (MCQs) based on the text. Each MCQ should have the following structure:

* `question`: The question itself, derived from the text.
* `options`: An array containing four possible answer choices.
* `answer`: The index (0-based) of the correct option in the `options` array.

**Example:**
{{
\"questions\": [
{{
\"question\": \"question 1\",
\"options\": [
\"Option 1\",
\"Option 2\",
\"Option 3\",
\"Option 4\"
],
\"answer\": 1
}},
{{
\"question\": \"question 2\",
\"options\": [
\"Option 1\",
\"Option 2\",
\"Option 3\",
\"Option 4\"
],
\"answer\": 4
}},
// ... and so on for {:?} questions
]
}}", topic, count, count, count );

    let gen_response: GenerateContentResponse = match generate_ai_content(prompt).await {
        Ok(s) => s,
        Err(error) => return Err(error),
    };

    let part = &gen_response.candidates[0].content.parts[0];

    match part {
        Part::Text(t) => return Ok(t.to_string()),
        _ => return Err("not the same type".to_string()),
    };
}
