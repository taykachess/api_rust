use api_db::{
    post::{Post, PostCreateDto},
    user::AuthUser,
};
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    middleware,
    routing::{delete, get, patch, post},
    Extension, Json, Router,
};

use serde::Deserialize;
use uuid::Uuid;

use crate::error::prelude::*;
use crate::{error::RouteResult, mw::jwt::Token};

#[derive(Deserialize)]
struct Pagination {
    page: usize,
    per_page: usize,
}

async fn check_author(user: &AuthUser, post_id: Uuid) -> RouteResult<()> {
    let post_from_db = Post::get_post(post_id).await?;
    ensure!(
        post_from_db.get_username() == user.username,
        StatusCode::FORBIDDEN
    );
    Ok(())
}

pub(crate) fn router() -> Router {
    Router::new()
        .route("/", post(create_post))
        .route("/:id", patch(update_post))
        .route("/:id", delete(delete_post))
        .layer(middleware::from_fn(crate::mw::jwt::token))
        .route("/", get(get_posts))
        .route("/:id", get(get_post))
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
    Extension(user): Extension<AuthUser>,
    Path(post_id): Path<Uuid>,
    Json(post): Json<Post>,
) -> RouteResult<Json<String>> {
    check_author(&user, post_id).await?;
    let post_id = Post::update_post(post, post_id).await?;
    Ok(Json(post_id))
}

async fn delete_post(
    Extension(user): Extension<AuthUser>,
    Path(post_id): Path<Uuid>,
) -> RouteResult<Json<String>> {
    check_author(&user, post_id).await?;
    let post_id = Post::delete_post(post_id).await?;
    Ok(Json(post_id))
}

async fn get_post(Path(post_id): Path<Uuid>) -> RouteResult<Json<Post>> {
    let post = Post::get_post(post_id).await?;

    Ok(Json(post))
}

async fn get_posts(Query(pagination): Query<Pagination>) -> RouteResult<Json<Vec<Post>>> {
    let limit = pagination.per_page;
    let offset = pagination.page * limit;
    let post = Post::get_posts(limit, offset).await?;

    Ok(Json(post))
}

#[cfg(test)]
mod tests {

    use crate::mw::jwt::authorize_current_user;

    use super::*;
    use api_db::{init_db_test, user::UserCredentialsDto};

    // #[tokio::test]
    // async fn create_post_test() {
    //     crate::test_utils::init_test_utils().await;
    //     let token = crate::test_utils::create_user_and_get_token().await;

    //     let user = authorize_current_user(token.get())
    //         .await
    //         .expect("fail to authorize user");

    //     let post_create_dto = PostCreateDto::new(Option::None, Option::Some("Body".to_owned()));

    //     let Json(post_id) = create_post(Extension(user.clone()), Json(post_create_dto))
    //         .await
    //         .expect("failed to create post");

    //     let Json(post) = get_post(Path(Uuid::try_parse(&post_id).expect("Not parced")))
    //         .await
    //         .expect("Not found Post");

    //     print!("Post: {:?}", post);

    //     let post_id = update_post(
    //         Extension(user.clone()),
    //         Path(Uuid::try_parse(&post_id).expect("Not parced")),
    //         Json(Post::new(
    //             PostCreateDto::new(Option::Some("Title".to_owned()), Option::None),
    //             user.username,
    //         )),
    //     )
    //     .await
    //     .expect("Post not updated");

    //     let Json(post) = get_post(Path(Uuid::try_parse(&post_id).expect("Not parced")))
    //         .await
    //         .expect("Not found Post");
    // }

    // #[tokio::test]
    // async fn crud_test() {
    //     crate::test_utils::init_test_utils().await;
    //     let token = crate::test_utils::create_user_and_get_token().await;

    //     let user = authorize_current_user(token.get())
    //         .await
    //         .expect("fail to authorize user");

    //     let post_create_dto = PostCreateDto::new(Option::None, Option::Some("Body".to_owned()));

    //     let Json(post_id) = create_post(Extension(user.clone()), Json(post_create_dto))
    //         .await
    //         .expect("failed to create post");

    //     let Json(post) = get_post(Path(Uuid::try_parse(&post_id).expect("Not parced")))
    //         .await
    //         .expect("Not found Post");

    //     let post_id = update_post(
    //         Extension(user.clone()),
    //         Path(Uuid::try_parse(&post_id).expect("Not parced")),
    //         Json(Post::new(
    //             PostCreateDto::new(Option::Some("Title".to_owned()), Option::None),
    //             user.username,
    //         )),
    //     )
    //     .await
    //     .expect("Post not updated");

    //     let Json(post) = get_post(Path(Uuid::try_parse(&post_id).expect("Not parced")))
    //         .await
    //         .expect("Not found Post");
    // }

    #[tokio::test]
    async fn get_posts_tests() {
        crate::test_utils::init_test_utils().await;
        let token = crate::test_utils::create_user_and_get_token().await;
        let user = authorize_current_user(token.get())
            .await
            .expect("fail to authorize user");

        for _ in 0..55 {
            let post_create_dto = PostCreateDto::new(Option::None, Option::Some("Body".to_owned()));
            let Json(id) = create_post(Extension(user.clone()), Json(post_create_dto))
                .await
                .expect("failed to create post");
        }

        let posts = get_posts(Query(Pagination {
            page: 0,
            per_page: 5,
        }))
        .await
        .expect("no posts");

        // assert!(posts.len() == 10);

        println!("Posts: {:?}", posts);
        println!("Posts: {}", posts.len());
    }
}
