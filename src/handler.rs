use std::env;

use crate::initialiser::Util;
use crate::model::{
    AIResponse, Content, CreateFlash, CreateQuiz, Faculty, Flashcard, GenerateContentRequest,
    GenerateContentResponse, GenerationConfig, GenericResponse, Part, Quiz, RequestAIQuery,
    Student, User,
};

use crate::helpers::{
    generate_ai_content, get_user_pwd, hasher, make_flashcards, make_quiz, verify,
};

use actix_web::web::Data;
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

#[post("/login")]
async fn login_user(
    db: web::Data<Database>,
    util: Data<Util>,
    form: web::Form<User>,
) -> impl Responder {
    let coll = db.collection::<Document>("users");

    let get_user_passwd = match get_user_pwd(form.username.clone(), coll).await {
        Ok(s) => s,
        Err(error) => return error.to_string(),
    };

    let res = match verify(form.password.clone(), get_user_passwd, util.Argon.clone()) {
        Ok(b) => {
            if b {
                let response_json = &GenericResponse {
                    status: "success".to_string(),
                    message: "user exists".to_string(),
                };
                HttpResponse::Ok().json(response_json)
            } else {
                let response_json = &GenericResponse {
                    status: "success".to_string(),
                    message: "user doesnot exists".to_string(),
                };
                HttpResponse::Ok().json(response_json)
            }
        }
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            HttpResponse::InternalServerError().json(response_json)
        }
    };

    "Login Successful".to_string()
}

#[post("/generate_flashcard")]
async fn generate_flashcard(MultipartForm(body): MultipartForm<RequestAIQuery>) -> impl Responder {
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
}}",
        body.count, body.content
    );

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

async fn generate_flashcard_tester(topic: String, count: i8) -> Result<String, String> {
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
async fn add_student(
    db: web::Data<Database>,
    util: Data<Util>,
    form: web::Form<User>,
) -> impl Responder {
    let coll = db.collection::<Document>("users");

    let pwd = match hasher(form.password.to_owned(), util.Argon.clone()) {
        Ok(s) => s,
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
        password: pwd,
        quiz: Some(Vec::new()),
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
async fn add_faculty(
    db: web::Data<Database>,
    util: Data<Util>,
    form: web::Form<User>,
) -> impl Responder {
    let coll = db.collection::<Document>("users");

    let pwd = match hasher(form.password.to_owned(), util.Argon.clone()) {
        Ok(s) => s,
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
        password: pwd,
        quiz: Some(Vec::new()),
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
async fn create_flash(db: web::Data<Database>, form: web::Form<CreateFlash>) -> impl Responder {
    let coll = db.collection::<Document>("users");
    let cont = match make_flashcards(form.topic.clone(), form.count).await {
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
        content: cont.clone(),
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

    if resp.modified_count == 0 {
        let response_json = &GenericResponse {
            status: "fail".to_string(),
            message: "Failed to update value".to_string(),
        };
        return HttpResponse::InternalServerError().json(response_json);
    }

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: cont,
    };

    HttpResponse::Ok().json(response_json)
}

#[post("/create_quiz")]
async fn create_quiz(db: web::Data<Database>, form: web::Form<CreateQuiz>) -> impl Responder {
    let coll = db.collection::<Document>("users");
    let cont = match make_quiz(form.topic.clone(), form.count).await {
        Ok(s) => s,
        Err(error) => {
            let response_json = &GenericResponse {
                status: "fail".to_string(),
                message: error.to_string(),
            };
            return HttpResponse::InternalServerError().json(response_json);
        }
    };

    let quiz = Quiz {
        _id: Uuid::new_v4().to_string(),
        quizzes: cont.clone(),
    };

    let bson_quiz = match to_document(&quiz) {
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
    let update = doc! { "$push": { "quiz": bson_quiz} };

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

    if resp.modified_count == 0 {
        let response_json = &GenericResponse {
            status: "fail".to_string(),
            message: "Failed to update value".to_string(),
        };
        return HttpResponse::InternalServerError().json(response_json);
    }

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: cont,
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
        .service(create_flash)
        .service(create_quiz)
        .service(login_user);

    conf.service(scope);
}
