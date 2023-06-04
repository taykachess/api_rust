use api_db::user::{AuthUser, User};
use axum::{routing::post, Json, Router};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::error::prelude::*;
use crate::mw::jwt::Token;

pub(crate) fn router() -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct UserDto {
    username: String,
    pass: String,
}

impl UserDto {
    #[cfg(test)]
    pub fn new(username: &str, pass: &str) -> Self {
        Self {
            username: username.to_owned(),
            pass: pass.to_owned(),
        }
    }
}

pub(crate) async fn signup(Json(user): Json<UserDto>) -> RouteResult<Json<Token>> {
    let hashed_password = tokio::task::spawn_blocking({
        let pass_input = user.pass.clone();
        move || {
            let hashed_password = pretty_sha2::sha512::gen(pass_input);
            hashed_password
        }
    })
    .await
    .map_err(|_| route_error!(NOT_FOUND, "Page not found."))?;

    User::new(&user.username, &hashed_password)
        .create_user()
        .await
        .map_err(|err| route_error!("Failed to create user {err}", INTERNAL_SERVER_ERROR))?;

    let token = tokio::task::spawn_blocking(move || {
        // FIXME env secrets
        encode(
            &Header::default(),
            &AuthUser::new(&user.username),
            &EncodingKey::from_secret("secret".as_ref()),
        )
    })
    .await
    .map_err(|_| {
        route_error!(
            "Failed to join spawn from encode token",
            INTERNAL_SERVER_ERROR
        )
    })?
    .map_err(|_| route_error!("Failed to encode token.", INTERNAL_SERVER_ERROR))?;

    Ok(Json(Token::new(token)))
}

pub(crate) async fn login(Json(user): Json<UserDto>) -> RouteResult<Json<Token>> {
    let user_from_db = api_db::user::User::get_user(&user.username)
        .await
        .map_err(|_| route_error!(" Failed to get user", INTERNAL_SERVER_ERROR))?
        .ok_or(route_error!("User not found", NOT_FOUND))?;

    let token = tokio::task::spawn_blocking(move || {
        let hashed_password = pretty_sha2::sha512::gen(&user.pass);
        if user_from_db.pass != hashed_password {
            throw!("Incorrect password", UNAUTHORIZED);
        };
        // FIXME env secrets
        encode(
            &Header::default(),
            &AuthUser::new(&user.username),
            &EncodingKey::from_secret("secret".as_ref()),
        )
        .map_err(|_| route_error!("Failed to encode token.", INTERNAL_SERVER_ERROR))
    })
    .await
    .map_err(|_| route_error!("Failed to encode token.", UNAUTHORIZED))??;

    Ok(Json(Token::new(token)))
}

#[cfg(test)]
mod tests {

    use super::*;
    use api_db::init_db_test;

    #[tokio::test]
    async fn login_test() {
        init_db_test().await.expect("failed to init db");

        //  Signup user with password
        let Json(_) = signup(Json(UserDto {
            username: "Vadim".to_string(),
            pass: "12345".to_string(),
        }))
        .await
        .expect("failed to signup");

        //  Right password
        let Json(token) = login(Json(UserDto {
            username: "Vadim".to_string(),
            pass: "12345".to_string(),
        }))
        .await
        .expect("failed to login");

        //  Wrong password!
        let res = login(Json(UserDto {
            username: "Vadim".to_string(),
            pass: "123456".to_string(),
        }))
        .await;

        // ensure!(res.is_err(), "Failed to login");

        assert!(matches!(res.is_err(), true));
    }
}
