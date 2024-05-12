use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// Define the AppState struct to hold the todolist entries
struct AppState {
    todolist_entries: Mutex<Vec<TodolistEntry>>,
}

// Define the TodolistEntry struct to represent a todolist entry
#[derive(Serialize, Deserialize)]
struct TodolistEntry {
    id: usize,
    title: String,
    description: String,
    date: i64,
}

#[get("/")]
async fn index() -> String {
    "Hello, world!".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let appData = web::Data::new(AppState {
        todolist_entries: Mutex::new(vec![]),
    });
    HttpServer::new(move || {
        App::new().app_data(appData.clone()).service(index)
        // .service(get_todolist_entries)
        // .service(add_todolist_entry)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
