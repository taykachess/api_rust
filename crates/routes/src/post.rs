use api_db::post::Post;
use axum::{
    extract::Path,
    http::StatusCode,
    routing::{delete, get, patch, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub(crate) fn router() -> Router {
    Router::new()
        .route("/", get(get_all_post))
        .route("/", post(create_post))
        .route("/:id", patch(update_post))
        .route("/:id", delete(delete_post))
}

#[derive(Deserialize, Serialize)]
struct PostDto {
    title: String,
    body: String,
}

async fn create_post(Json(post_dto): Json<PostDto>) -> Result<Json<String>, StatusCode> {
    // TODO: extract author via middleware username
    let post = Post::new("vadim", &post_dto.title, &post_dto.body);
    let post_id = post
        .create_post()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(post_id))
}

// WOW ordering of extractors is important!
async fn update_post(
    Path(post_id): Path<Uuid>,
    Json(post): Json<Post>,
) -> Result<Json<String>, StatusCode> {
    Post::update_post(post, post_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json("StatusCode::OK".to_owned()))
}

async fn delete_post() {}

async fn get_all_post() {
    // TODO: Filters and Pagintation
}

#[cfg(test)]
mod tests {

    use crate::auth::UserDto;

    use super::*;
    use api_db::init_db_test;

    #[tokio::test]
    async fn create_post_test() {
        init_db_test().await.expect("failed to init db");

        //  Signup user with password
        let Json(_) = crate::auth::signup(Json(UserDto::new("Vadim", "12345")))
            .await
            .expect("failed to signup");

        //  login
        let Json(token) = crate::auth::login(Json(UserDto::new("Vadim", "12345")))
            .await
            .expect("failed to login");

        // TODO: use token for authentication

        println!("{token:?}");

        let post_dto = PostDto {
            title: "test".to_owned(),
            body: "test".to_owned(),
        };

        let post = create_post(Json(post_dto))
            .await
            .expect("failed to create post");

        println!("{:?}", post);
    }
}
