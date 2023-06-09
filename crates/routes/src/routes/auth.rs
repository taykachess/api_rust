use api_db::user::{AuthUser, User, UserCredentialsDto};
use axum::{routing::post, Json, Router};
use http::StatusCode;
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::error::prelude::*;
use crate::mw::jwt::Token;
use std::env;

pub(crate) fn router() -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
}

pub(crate) async fn signup(Json(user): Json<UserCredentialsDto>) -> RouteResult<Json<Token>> {
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
        .context("Failed to create user")
        .status_code(StatusCode::INTERNAL_SERVER_ERROR)?;

    let secret = env::var("JWT_SECRET")?;

    let token = tokio::task::spawn_blocking(move || {
        encode(
            &Header::default(),
            &AuthUser::new(&user.username),
            &EncodingKey::from_secret(secret.as_ref()),
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

pub(crate) async fn login(Json(user): Json<UserCredentialsDto>) -> RouteResult<Json<Token>> {
    let user_from_db = api_db::user::User::get_user(&user.username)
        .await
        .map_err(|_| route_error!(" Failed to get user", INTERNAL_SERVER_ERROR))?
        .ok_or(route_error!("User not found", NOT_FOUND))?;
    // ensure!(false);
    println!("{:?}", route_error!("User not found", NOT_FOUND));

    let token = tokio::task::spawn_blocking(move || {
        let hashed_password = pretty_sha2::sha512::gen(&user.pass);
        let secret = env::var("JWT_SECRET")?;

        ensure!(
            user_from_db.pass == hashed_password,
            "Incorrect password",
            UNAUTHORIZED
        );
        encode(
            &Header::default(),
            &AuthUser::new(&user.username),
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .context("Failed to encode token.")
        .status_code(StatusCode::INTERNAL_SERVER_ERROR)
    })
    .await
    .context("Failed to encode token.")
    .status_code(StatusCode::UNAUTHORIZED)??;
    Ok(Json(Token::new(token)))
}

#[cfg(test)]
mod tests {

    use super::*;
    use api_db::init_db_test;

    #[tokio::test]
    async fn login_test() {
        dotenv::dotenv().ok();
        init_db_test().await.expect("failed to init db");

        //  Signup user with password
        let Json(_) = signup(Json(UserCredentialsDto {
            username: "Vadim".to_string(),
            pass: "12345".to_string(),
        }))
        .await
        .expect("failed to signup");

        //  Right password
        let Json(token) = login(Json(UserCredentialsDto {
            username: "Vadim".to_string(),
            pass: "12345".to_string(),
        }))
        .await
        .expect("failed to login");

        //  Wrong password!
        let res = login(Json(UserCredentialsDto {
            username: "Vadim".to_string(),
            pass: "123456".to_string(),
        }))
        .await;

        // ensure!(res.is_err(), "Failed to login");

        assert!(matches!(res.is_err(), true));
    }
}
