use api_db::user::User;
use axum::{body::Body, routing::post, Json, Router};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

pub(crate) fn router() -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
}

#[derive(Deserialize, Serialize, Clone)]
struct UserDto {
    username: String,
    pass: String,
}

async fn signup(Json(user): Json<UserDto>) -> Json<String> {
    let hashed_password = pretty_sha2::sha512::gen(&user.pass);
    api_db::user::User::create_user(&User::new(&user.username, &hashed_password))
        .await
        .expect("Unable to create user");

    let token = encode(
        &Header::default(),
        &user,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .expect("parse problem");

    Json(token)
}

async fn login(Json(user): Json<UserDto>) {
    api_db::user::User::get_user(&user.username).await;
}
