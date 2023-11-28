use axum::Json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RequestUser {
    username: String,
    password: String,
}

pub async fn validate_data(Json(user): Json<RequestUser>) {
    dbg!(user);
}
