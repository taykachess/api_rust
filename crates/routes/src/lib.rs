use axum::{routing::get, Router};

pub mod auth;

pub async fn init_server(addr: std::net::SocketAddr) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!!" }))
        .nest("/auth", auth::router());

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
