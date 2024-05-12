use super::models::{CreateEntryData, UpdateEntryData};
use crate::{AppState, TodolistEntry};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/todolist/entries")]
async fn get_entries(data::web: Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.todolist_entries.lock().unwrap().to_vec())
}

#[post("/todolist/entries")]
async fn create_entry(
    data::web: Data<AppState>,
    param_object: web::Json<CreateEntryData>,
) -> impl Responder {
}
