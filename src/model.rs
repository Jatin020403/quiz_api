use actix_multipart::{
    form::{
        tempfile::{TempFile, TempFileConfig},
        MultipartForm,
    },
    Multipart,
};

use actix_multipart::form::text::Text;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// REQUESTS

#[derive(MultipartForm)]
pub struct FileUpload {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[derive(Debug, MultipartForm)]
pub struct RequestAIQuery {
    pub id: Option<Text<String>>,
    pub timestamp: Option<Text<DateTime<Utc>>>,
    pub content_type: Text<String>,
    pub request: Text<String>,
    pub content: Text<String>,
    pub count: Text<i8>,
    pub files: Option<TempFile>,
}

#[derive(Serialize, Deserialize)]
pub struct GenerateContentRequest {
    pub contents: Vec<Content>,
    pub generation_config: Option<GenerationConfig>,
    pub tools: Option<Vec<Tools>>,
}

// RESPONSE

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AIResponse {
    pub status: String,
    pub response: GenerateContentResponse,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentResponse {
    pub candidates: Vec<Candidate>,
    pub usage_metadata: Option<UsageMetadata>,
}

// QUIZ/FLASHCARDS

#[derive(Serialize, Deserialize, Debug)]
pub struct Faculty {
    pub faculty_id: String,
    pub name: String,
    pub quiz_id: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Student {
    pub student_id: String,
    pub name: String,
    pub quiz_id: Vec<String>,
    pub flash_id: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Admin {
    pub admin_id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Flashcard {
    pub flash_id: String,
    pub student_id: String,
    pub cards: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quiz {
    pub quiz_id: String,
    pub faculty_id: String,
    pub questions: Vec<Vec<String>>,
    pub answers: Vec<i32>,
    pub student_answers: HashMap<String, Vec<i32>>,
    pub student_marks: HashMap<String, i32>,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LLM {
    pub req_id: String,
    pub purpose: String,
    pub msg: String,
    pub created_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
}

// GEMINI VERTEX STRUCTS

#[derive(Debug, Serialize, Deserialize)]
pub struct CountTokensRequest {
    pub contents: Content,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountTokensResponse {
    pub total_tokens: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Tools {
    pub function_declarations: Option<Vec<FunctionDeclaration>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub role: String,
    pub parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenerationConfig {
    pub max_output_tokens: Option<i32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<i32>,
    pub stop_sequences: Option<Vec<String>>,
    pub candidate_count: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Part {
    Text(String),
    InlineData {
        mime_type: String,
        data: String,
    },
    FileData {
        mime_type: String,
        file_uri: String,
    },
    FunctionCall {
        name: String,
        args: HashMap<String, String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    pub content: Content,
    pub citation_metadata: Option<CitationMetadata>,
    pub safety_ratings: Vec<SafetyRating>,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SafetyRating {
    pub category: String,
    pub probability: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Citation {
    start_index: i32,
    end_index: i32,
    uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CitationMetadata {
    pub citations: Vec<Citation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageMetadata {
    candidates_token_count: Option<i32>,
    prompt_token_count: i32,
    total_token_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionDeclaration {
    pub name: String,
    pub description: String,
    pub parameters: FunctionParameters,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionParameters {
    pub r#type: String,
    pub properties: HashMap<String, FunctionParametersProperty>,
    pub required: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionParametersProperty {
    pub r#type: String,
    pub description: String,
}
