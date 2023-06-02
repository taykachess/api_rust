use anyhow::Ok;
use axum::{
    extract::Extension,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub(crate) struct Token {
    token: String,
}

impl Token {
    pub(crate) fn new(token: String) -> Self {
        Self { token }
    }
}

pub(crate) async fn token<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    println!("Middleware");
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Some(token) = authorize_current_user(auth_header).await {
        // insert the current user into a request extension so the handler can
        // extract it
        req.extensions_mut().insert(token);
        Ok(next.run(req).await).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn authorize_current_user(token: &str) -> Option<Token> {
    Some(Token::new("token".to_owned()))
    // ...
}

// async fn handler(
//     // extract the current user, set by the middleware
//     Extension(current_user): Extension<CurrentUser>,
// ) {
//     // ...
// }

// let app = Router::new()
//     .route("/", get(handler))
//     .route_layer(middleware::from_fn(auth));
