use std::env;

use crate::model::{
    AIResponse, Content, GenerateContentRequest, GenerateContentResponse, GenerationConfig,
    GenericResponse, Part, RequestAIQuery,
};
use gcp_auth::AuthenticationManager;

use actix_multipart::{
    form::{
        tempfile::{TempFile, TempFileConfig},
        MultipartForm,
    },
    Multipart,
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use uuid::Uuid;

static MODEL_NAME: &str = "gemini-pro";

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "All Ok";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}

/*
#[get("/generate_flashcard")]
async fn generate_flashcard() -> impl Responder {
    let api_endpoint: String = match env::var("API_ENDPOINT") {
        Ok(s) => s,
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };
    let project_id: String = match env::var("PROJECT_ID") {
        Ok(s) => s,
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };
    let location_id: String = match env::var("LOCATION_ID") {
        Ok(s) => s,
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    }; // Sometimes called "region" in gCloud docs.

    println!("Endpoint : {:?}", api_endpoint);

    let endpoint_url = format!(
        "https://{api_endpoint}/v1beta1/projects/{project_id}/locations/{location_id}/publishers/google/models/{MODEL_NAME}:generateContent"
    );

    let authentication_manager: AuthenticationManager = match AuthenticationManager::new().await {
        Ok(s) => s,
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };
    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let token = match authentication_manager.get_token(scopes).await {
        Ok(s) => s,
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };

    let prompt = "Extract key points from the text

Present the information in a JSON format with two fields:
* key_points_array: An array containing each key point as a string.
* number_of_key_points: The number of elements in the key_points_array.

Use only these valid fields.

Text: There was once a family of Five Little Pigs, and Mrs. Pig, their mother, loved them all very dearly. Some of these little pigs were very good, and took a great deal of trouble to please her. The eldest pig was so active and useful that he was called Mr. Pig. One day he went to market with his cart full of vegetables, but Rusty, the donkey, began to show his bad temper before he had gone very far on the road. All the coaxing and whipping would not make him move. So Mr. Pig took him out of the shafts, and being very strong, drew the cart to market himself. When he got there, all the other pigs began to laugh. But they did not laugh so loudly when Mr. Pig told them all his struggles on the road. Mr. Pig lost no time in selling his vegetables, and very soon after Rusty came trotting into the market-place, and as he now seemed willing to take his place in the cart, Mr. Pig started for home without delay. When he got there, he told Mrs. Pig his story, and she called him her best and most worthy son.

JSON:";

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
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };

    let gen_response: GenerateContentResponse = match resp.json::<GenerateContentResponse>().await {
        Ok(s) => s,
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };

    let response_json = &AIResponse {
        status: "success".to_string(),
        response: gen_response,
    };

    HttpResponse::Ok().json(response_json)
}
*/

#[post("/generate_flashcard")]
// (web::Form(body): web::Form<RequestAIQuery>)
// MultipartForm(form): MultipartForm<UploadForm>,
async fn generate_flashcard(MultipartForm(body): MultipartForm<RequestAIQuery>) -> impl Responder {
    let api_endpoint: String = match env::var("API_ENDPOINT") {
        Ok(s) => s,
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };
    let project_id: String = match env::var("PROJECT_ID") {
        Ok(s) => s,
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };
    let location_id: String = match env::var("LOCATION_ID") {
        Ok(s) => s,
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    }; // Sometimes called "region" in gCloud docs.

    println!("Endpoint : {:?}", api_endpoint);

    let endpoint_url = format!(
        "https://{api_endpoint}/v1beta1/projects/{project_id}/locations/{location_id}/publishers/google/models/{MODEL_NAME}:generateContent"
    );

    let authentication_manager: AuthenticationManager = match AuthenticationManager::new().await {
        Ok(s) => s,
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };
    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let token = match authentication_manager.get_token(scopes).await {
        Ok(s) => s,
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };

    let prompt = format!(
        "Extract {:?} key points from the text
Present the information in a JSON format with two fields:
* key_points_array: An array containing each key point as a string.
* number_of_key_points: The number of elements in the key_points_array.
Use only these valid fields.

Text: {:?}

JSON:",
        body.count, body.content
    );

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
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };

    let gen_response: GenerateContentResponse = match resp.json::<GenerateContentResponse>().await {
        Ok(s) => s,
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };

    let response_json = &AIResponse {
        status: "success".to_string(),
        response: gen_response,
    };

    HttpResponse::Ok().json(response_json)
}

#[post("/generate_quiz")]
async fn generate_quiz(MultipartForm(body): MultipartForm<RequestAIQuery>) -> impl Responder {
    let api_endpoint: String = match env::var("API_ENDPOINT") {
        Ok(s) => s,
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };
    let project_id: String = match env::var("PROJECT_ID") {
        Ok(s) => s,
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };
    let location_id: String = match env::var("LOCATION_ID") {
        Ok(s) => s,
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    }; // Sometimes called "region" in gCloud docs.

    println!("Endpoint : {:?}", api_endpoint);

    let endpoint_url = format!(
        "https://{api_endpoint}/v1beta1/projects/{project_id}/locations/{location_id}/publishers/google/models/{MODEL_NAME}:generateContent"
    );

    let authentication_manager: AuthenticationManager = match AuthenticationManager::new().await {
        Ok(s) => s,
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };
    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let token = match authentication_manager.get_token(scopes).await {
        Ok(s) => s,
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };

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
}}", body.content, body.count, body.count, body.count );

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
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };

    let gen_response: GenerateContentResponse = match resp.json::<GenerateContentResponse>().await {
        Ok(s) => s,
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };

    let response_json = &AIResponse {
        status: "success".to_string(),
        response: gen_response,
    };

    HttpResponse::Ok().json(response_json)
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(generate_flashcard)
        .service(generate_quiz);

    conf.service(scope);
}
