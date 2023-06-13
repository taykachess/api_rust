use axum::{middleware, routing::get, Router};
use mw::log::logger_route;
mod error;

mod mw;
mod routes;

#[cfg(test)]
mod test_utils;

pub async fn init_server(addr: std::net::SocketAddr) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!!" }))
        .nest("/auth", routes::auth::router())
        .nest("/post", routes::post::router())
        .layer(middleware::from_fn(logger_route));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
