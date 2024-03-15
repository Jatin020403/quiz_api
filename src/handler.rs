use crate::response::GenericResponse;

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use uuid::Uuid;

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "All Ok";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api").service(health_checker_handler);

    conf.service(scope);
}
