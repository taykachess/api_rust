use std::env;

use api_db::user::AuthUser;
use axum::{http::Request, middleware::Next, response::Response};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;

use crate::error::prelude::*;

#[derive(Serialize, Debug, Clone)]
pub(crate) struct Token {
    token: String,
}

impl Token {
    pub(crate) fn new(token: String) -> Self {
        Self { token }
    }

    #[cfg(test)]
    pub fn get(&self) -> &str {
        &self.token
    }
}

pub(crate) async fn token<B>(mut req: Request<B>, next: Next<B>) -> RouteResult<Response> {
    // transform option into
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(route_error!(UNAUTHORIZED, "Auth header not found."));
    };

    let auth_header = auth_header.split_whitespace().collect::<Vec<_>>();
    if auth_header.len() != 2 {
        return Err(route_error!(UNAUTHORIZED, "Wrong parcing auth header."));
    }

    let token = auth_header[1];

    if let Ok(user) = authorize_current_user(token).await {
        req.extensions_mut().insert(user);
        let res = next.run(req).await;
        Ok(res)
    } else {
        Err(route_error!(UNAUTHORIZED, "Can't extract user."))
    }
}

pub(crate) async fn authorize_current_user(token: &str) -> RouteResult<AuthUser> {
    let token = token.to_owned();
    let secret = env::var("JWT_SECRET")?;

    let user = tokio::task::spawn_blocking(move || {
        decode::<AuthUser>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )
    })
    .await
    .context("Wrong decoding token.")?
    .context("Wrong decoding token.")?;

    Ok(user.claims)
}
