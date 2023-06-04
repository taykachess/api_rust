use api_db::{
    post::{Post, PostCreateDto},
    user::AuthUser,
};
use axum::{
    extract::Path,
    http::StatusCode,
    middleware,
    routing::{delete, get, patch, post},
    Extension, Json, Router,
};

use uuid::Uuid;

use crate::mw::jwt::Token;

pub(crate) fn router() -> Router {
    Router::new()
        .route("/", post(create_post))
        // .route("/", get(get_all_post))
        // .route("/:id", patch(update_post))
        // .route("/:id", delete(delete_post))
        .route_layer(middleware::from_fn(crate::mw::jwt::token))
}

async fn create_post(
    Extension(user): Extension<AuthUser>,
    Json(post_create_dto): Json<PostCreateDto>,
) -> Result<Json<String>, StatusCode> {
    let post = Post::new(post_create_dto, user.username);
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
) -> Result<StatusCode, StatusCode> {
    Post::update_post(post, post_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn delete_post(Path(post_id): Path<Uuid>) -> Result<(), StatusCode> {
    Post::delete_post(post_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

async fn get_all_post() {
    todo!()
}

#[cfg(test)]
mod tests {

    use crate::{mw::jwt::authorize_current_user, routes::auth::UserDto};

    use super::*;
    use api_db::init_db_test;

    #[tokio::test]
    async fn create_post_test() {
        init_db_test().await.expect("failed to init db");

        // TODO: DRY
        //  Signup user with password
        let Json(_) = crate::routes::auth::signup(Json(UserDto::new("Vadim123", "12345")))
            .await
            .expect("failed to signup");

        //  login
        let Json(token) = crate::routes::auth::login(Json(UserDto::new("Vadim123", "12345")))
            .await
            .expect("failed to login");

        // TODO: use token for authentication

        let user = authorize_current_user(token.get())
            .await
            .expect("fail to authorize user");

        let post_create_dto = PostCreateDto::new(&user.username, "title", "body");

        let _ = create_post(Json(post_create_dto))
            .await
            .expect("failed to create post");
    }
}
