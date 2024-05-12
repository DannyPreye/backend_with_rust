use serde::Deserialize;

#[derive{Deserialize, Clone}]
pub struct CreateEntryData {
    pub title: String,
    pub date: i64,
    pub description: String,
}

#[derive{Deserialize, Clone}]
pub struct UpdateEntryData {
    pub title: Option<String>,
    pub date: Option<i64>,
    pub description: Option<String>,
}
