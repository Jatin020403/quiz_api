use std::env;

use crate::model::{
    AIResponse, Content, CreateFlash, CreateUser, Faculty, Flashcard, GenerateContentRequest,
    GenerateContentResponse, GenerationConfig, GenericResponse, Part, RequestAIQuery, Student,
};

use crate::helpers::generate_ai_content;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

use bson::{doc, to_document};
use gcp_auth::AuthenticationManager;

use actix_multipart::form::MultipartForm;
use actix_web::{get, post, web, HttpResponse, Responder};

extern crate mongodb;
// use chrono::prelude::*;
use mongodb::{bson::Document, Database};
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

#[post("/generate_flashcard")]
async fn generate_flashcard(MultipartForm(body): MultipartForm<RequestAIQuery>) -> impl Responder {

    let prompt = format!(
        "Extract {:?} key points from the text. Present the information in a JSON format with two fields:
* key_points_array: An array containing each key point as a string.
* number_of_key_points: The number of elements in the key_points_array.
Use only these valid fields.

Text: {:?}

JSON:",
        body.count, body.content
    );

    let gen_response: GenerateContentResponse = match generate_ai_content(prompt).await{
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

async fn generate_flashcard_tester(topic: String, count: i8) -> Result<String, String> {
    let api_endpoint: String = match env::var("API_ENDPOINT") {
        Ok(s) => s,
        Err(error) => return Err(error.to_string()),
    };
    let project_id: String = match env::var("PROJECT_ID") {
        Ok(s) => s,
        Err(error) => return Err(error.to_string()),
    };
    let location_id: String = match env::var("LOCATION_ID") {
        Ok(s) => s,
        Err(error) => return Err(error.to_string()),
    }; // Sometimes called "region" in gCloud docs.

    println!("Endpoint : {:?}", api_endpoint);

    let endpoint_url = format!(
        "https://{api_endpoint}/v1beta1/projects/{project_id}/locations/{location_id}/publishers/google/models/{MODEL_NAME}:generateContent"
    );

    let authentication_manager: AuthenticationManager = match AuthenticationManager::new().await {
        Ok(s) => s,
        Err(error) => return Err(error.to_string()),
    };
    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let token = match authentication_manager.get_token(scopes).await {
        Ok(s) => s,
        Err(error) => return Err(error.to_string()),
    };

    let prompt = format!(
        "Extract {:?} key points from the text
Present the information in a JSON format with two fields:
* key_points_array: An array containing each key point as a string.
* number_of_key_points: The number of elements in the key_points_array.
Use only these valid fields.

Text: {:?}

JSON:",
        count, topic
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
        Err(error) => return Err(error.to_string()),
    };

    let gen_response: GenerateContentResponse = match resp.json::<GenerateContentResponse>().await {
        Ok(s) => s,
        Err(error) => return Err(error.to_string()),
    };

    let part = &gen_response.candidates[0].content.parts[0];

    match part {
        Part::Text(t) => return Ok(t.to_string()),
        _ => return Err("not the same type".to_string()),
    };
}

#[post("/generate_quiz")]
async fn generate_quiz(MultipartForm(body): MultipartForm<RequestAIQuery>) -> impl Responder {

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

    let gen_response: GenerateContentResponse = match generate_ai_content(prompt).await {
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

#[post("/add_student")]
async fn add_student(db: web::Data<Database>, form: web::Form<CreateUser>) -> impl Responder {
    let coll = db.collection::<Document>("users");

    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = match argon2.hash_password(form.password.as_bytes(), &salt) {
        Ok(s) => s.to_string(),
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };
    let user = Student {
        _id: Uuid::new_v4().to_string(),
        username: form.username.clone(),
        password: password_hash,
        quiz_id: Some(Vec::new()),
        flashes: Some(Vec::new()),
    };

    let bson_user = match to_document(&user) {
        Ok(s) => s,
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };

    let resp = match coll.insert_one(bson_user, None).await {
        Ok(s) => s,
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: resp.inserted_id.to_string(),
    };

    HttpResponse::Ok().json(response_json)
}

#[post("/add_faculty")]
async fn add_faculty(db: web::Data<Database>, form: web::Form<CreateUser>) -> impl Responder {
    let coll = db.collection::<Document>("users");

    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = match argon2.hash_password(form.password.as_bytes(), &salt) {
        Ok(s) => s.to_string(),
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };

    let user = Faculty {
        _id: Uuid::new_v4().to_string(),
        username: form.username.clone(),
        password: password_hash,
        quiz_id: Some(Vec::new()),
        flashes: Some(Vec::new()),
    };

    let bson_user = match to_document(&user) {
        Ok(s) => s,
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };

    let resp = match coll.insert_one(bson_user, None).await {
        Ok(s) => s,
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: resp.inserted_id.to_string(),
    };

    HttpResponse::Ok().json(response_json)
}

#[post("/create_flash")]
async fn create_flash(db: web::Data<Database>, focreate_flarm: web::Form<CreateFlash>) -> impl Responder {
    let coll = db.collection::<Document>("users");
    let cont = match generate_flashcard_tester(form.topic.clone(), form.count).await {
        Ok(s) => s,
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };

    let flash = Flashcard {
        _id: Uuid::new_v4().to_string(),
        topic: form.topic.clone(),
        content: cont,
    };

    let bson_flash = match to_document(&flash) {
        Ok(s) => s,
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };

    let filter = doc! { "_id": &form.user_id };
    let update = doc! { "$push": { "flashes": bson_flash} };

    let resp = match coll.update_one(filter, update, None).await {
        Ok(s) => s,
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: resp.modified_count.to_string(),
    };

    HttpResponse::Ok().json(response_json)
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(generate_flashcard)
        .service(generate_quiz)
        .service(add_student)
        .service(add_faculty)
        .service(create_flash);

    conf.service(scope);
}
