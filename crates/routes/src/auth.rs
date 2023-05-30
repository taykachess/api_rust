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
pub(crate) struct UserDto {
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
pub(crate) struct Token {
    token: String,
}

pub(crate) async fn signup(Json(user): Json<UserDto>) -> Result<Json<Token>, StatusCode> {
    // TODO: move to separate spawn
    let hashed_password = pretty_sha2::sha512::gen(&user.pass);

    println!("1");

    User::new(&user.username, &hashed_password)
        .create_user()
        .await
        .map_err(|err| {
            println!("{err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

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

pub(crate) async fn login(Json(user): Json<UserDto>) -> Result<Json<Token>, StatusCode> {
    let user_from_db = api_db::user::User::get_user(&user.username)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // TODO: move to separate spawn

    let hashed_password = pretty_sha2::sha512::gen(&user.pass);
    if user_from_db.pass != hashed_password {
        return Err(StatusCode::UNAUTHORIZED);
    }

    tokio::task::spawn_blocking({
        let pass_from_db = user_from_db.pass.clone();
        let pass_input = user.pass.clone();
        move || {
            let hashed_password = pretty_sha2::sha512::gen(pass_input);
            if pass_from_db != hashed_password {
                return Err(StatusCode::UNAUTHORIZED);
            }
            Ok(hashed_password)
        }
    })
    .await
    .map_err(|_| StatusCode::UNAUTHORIZED)?
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let token = tokio::task::spawn_blocking(move || {
        encode(
            &Header::default(),
            &user,
            &EncodingKey::from_secret("secret".as_ref()),
        )
    })
    .await
    .map_err(|_| StatusCode::UNAUTHORIZED)?
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(Json(Token { token }))
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

        println!("{token:?}");

        //  Wrong password!
        let res = login(Json(UserDto {
            username: "Vadim".to_string(),
            pass: "123456".to_string(),
        }))
        .await;
        assert!(res.is_err());
    }
}
