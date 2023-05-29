use api_db::user::User;
use axum::{http::StatusCode, routing::post, Json, Router};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

pub(crate) fn router() -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UserDto {
    username: String,
    pass: String,
}

impl UserDto {
    pub fn new(username: &str, pass: &str) -> Self {
        Self {
            username: username.to_owned(),
            pass: pass.to_owned(),
        }
    }
}

#[derive(Serialize, Debug)]
struct Token {
    token: String,
}

async fn signup(Json(user): Json<UserDto>) -> Result<Json<Token>, StatusCode> {
    // TODO! move to separate spawn
    let hashed_password = pretty_sha2::sha512::gen(&user.pass);

    User::new(&user.username, &hashed_password)
        .create_user()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let token = tokio::task::spawn_blocking(move || {
        encode(
            &Header::default(),
            &user,
            &EncodingKey::from_secret("secret".as_ref()),
        )
    })
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(Token { token }))
}

pub async fn login(Json(user): Json<UserDto>) -> Result<String, StatusCode> {
    let user_from_db = api_db::user::User::get_user(&user.username)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // TODO! move to separate spawn

    let hashed_password = pretty_sha2::sha512::gen(&user.pass);

    if user_from_db.pass != hashed_password {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = encode(
        &Header::default(),
        &user,
        &EncodingKey::from_secret("secret".as_ref()),
    );

    let token = match token {
        Ok(jwt) => jwt,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    // TODO! move to separate spawn

    Ok(token)
}
