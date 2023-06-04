use api_db::user::AuthUser;
use axum::{http::Request, middleware::Next, response::Response};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;

use crate::error::{route_error, RouteResult};

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

    // let user = authorize_current_user(token)
    //     .await
    //     .expect("fail to authorize user");

    if let Ok(user) = authorize_current_user(token).await {
        // insert the current user into a request extension so the handler can
        // extract it
        println!("user: {:?}", user);
        req.extensions_mut().insert(user);

        let res = next.run(req).await;
        println!("ok");
        Ok(res)
    } else {
        Err(route_error!(UNAUTHORIZED, "Can't extract user."))
    }
}

// TODO can I get rid of "async" ?  Replace spawn::blocking ?
pub async fn authorize_current_user(token: &str) -> RouteResult<AuthUser> {
    let user = decode::<AuthUser>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    )
    .map_err(|_| route_error!(NOT_FOUND, "Page not found."))?;

    Ok(user.claims)

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
