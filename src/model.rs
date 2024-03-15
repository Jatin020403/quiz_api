use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub msg:String,
    pub created_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
}