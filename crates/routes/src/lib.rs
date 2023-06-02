use axum::{routing::get, Router};

mod mw;
mod routes;

// TODO Create custom Error type for routes

pub async fn init_server(addr: std::net::SocketAddr) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!!" }))
        .nest("/auth", routes::auth::router())
        .nest("/post", routes::post::router());

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
