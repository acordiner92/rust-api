use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    message: String,
    count: i32,
    username: String,
}

pub async fn get_json() -> Json<Data> {
    Json(Data {
        message: "hello".to_string(),
        count: 321561,
        username: "cordo".to_string(),
    })
}
