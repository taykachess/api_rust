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
        .route("/", get(get_post))
        .route("/:id", patch(update_post))
        .route("/:id", delete(delete_post))
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
) -> Result<String, StatusCode> {
    let post_id = Post::update_post(post, post_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(post_id)
}

async fn delete_post(Path(post_id): Path<Uuid>) -> Result<String, StatusCode> {
    let post_id = Post::delete_post(post_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(post_id)
}

async fn get_post(Path(post_id): Path<Uuid>) -> Result<Json<Post>, StatusCode> {
    let post = Post::get_post(post_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(post))
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

        //  Signup
        let Json(_) = crate::routes::auth::signup(Json(UserDto::new("Vadim123", "12345")))
            .await
            .expect("failed to signup");

        //  login
        let Json(token) = crate::routes::auth::login(Json(UserDto::new("Vadim123", "12345")))
            .await
            .expect("failed to login");

        let user = authorize_current_user(token.get())
            .await
            .expect("fail to authorize user");

        let post_create_dto = PostCreateDto::new(Option::None, Option::Some("Body".to_owned()));

        let Json(post_id) = create_post(Extension(user.clone()), Json(post_create_dto))
            .await
            .expect("failed to create post");

        println!("Post id , {post_id}");
        let Json(post) = get_post(Path(Uuid::try_parse(&post_id).expect("Not parced")))
            .await
            .expect("Not found Post");
        println!("{post:?}");

        let post_id_thing = update_post(
            Path(Uuid::try_parse(&post_id).expect("Not parced")),
            Json(Post::new(
                PostCreateDto::new(Option::Some("Title".to_owned()), Option::None),
                user.username,
            )),
        )
        .await
        .expect("Post not updated");

        // TODO привести все id к одному виду
        println!("Post id , {post_id_thing}");
        let Json(post) = get_post(Path(Uuid::try_parse(&post_id).expect("Not parced")))
            .await
            .expect("Not found Post");

        println!("{post:?}");
    }
}
