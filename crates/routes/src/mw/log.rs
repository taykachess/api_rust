use axum::{http::Request, middleware::Next, response::Response};

use crate::error::RouteResult;

pub(crate) async fn logger_route<B>(mut req: Request<B>, next: Next<B>) -> RouteResult<Response> {
    let method = req.method().clone();
    let uri = req.uri().to_string();
    let res = next.run(req).await;

// TODO USE log  (initialize log in main)
    println!("{} {} {}", method, uri, res.status());

    Ok(res)
}
