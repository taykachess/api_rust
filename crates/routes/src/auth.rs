use axum::{body::Body, routing::post, Json, Router};
use serde::Deserialize;

pub(crate) fn router() -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
}

#[derive(Deserialize)]
struct UserDto {
    username: String,
    pass: String,
}

async fn signup(Json(user): Json<UserDto>) {}

async fn login() {}
