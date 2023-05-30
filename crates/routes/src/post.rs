use api_db::user::User;
use axum::{
    http::StatusCode,
    routing::{delete, get, patch, post},
    Json, Router,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

pub(crate) fn router() -> Router {
    Router::new()
        .route("/", get(get_all_post))
        .route("/", post(create_post))
        .route("/:id", patch(update_post))
        .route("/:id", delete(delete_post))
}

async fn create_post() -> Json<String> {
    Json('s'.to_string())
}

async fn update_post() {}

async fn delete_post() {}

async fn get_all_post() {
    // TODO! Filters and Pagintation
}
