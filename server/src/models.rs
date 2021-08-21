use serde::Serialize;

#[derive(Serialize)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub url: String,
}
