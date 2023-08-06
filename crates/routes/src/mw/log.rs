use axum::{http::Request, middleware::Next, response::Response};
use log::info;

use crate::error::RouteResult;

pub(crate) async fn logger_route<B>(req: Request<B>, next: Next<B>) -> RouteResult<Response> {
    let method = req.method().clone();
    let uri = req.uri().to_string();
    let res = next.run(req).await;

    info!("{} {} {}", method, uri, res.status());

    Ok(res)
}
