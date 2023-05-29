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

pub async fn signup(Json(user): Json<UserDto>) -> Result<String, StatusCode> {
    let hashed_password = pretty_sha2::sha512::gen(&user.pass);

    let created_user =
        api_db::user::User::create_user(&User::new(&user.username, &hashed_password)).await;

    match created_user {
        Ok(_) => (),
        Err(_) => return Err(StatusCode::NOT_FOUND),
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

    Ok(token)
}

pub async fn login(Json(user): Json<UserDto>) -> Result<String, StatusCode> {
    let user_from_db = match api_db::user::User::get_user(&user.username).await {
        Ok(option_user) => match option_user {
            Some(user) => user,
            None => return Err(StatusCode::NOT_FOUND),
        },
        Err(_) => return Err(StatusCode::SERVICE_UNAVAILABLE),
    };

    let hashed_password = pretty_sha2::sha512::gen(&user.pass);

    if user_from_db.pass != hashed_password {
        return Err(StatusCode::BAD_REQUEST);
    }

    let token = encode(
        &Header::default(),
        &user,
        &EncodingKey::from_secret("secret".as_ref()),
    );
    // .or_else(|_| Err(StatusCode::INTERNAL_SERVER_ERROR));

    let token = match token {
        Ok(jwt) => jwt,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(token)
}
