#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let addr_db = "127.0.0.1:8000";
    let ns = "test";
    let db = "test";
    api_db::init_db(ns, db).await?;

    let addr = "0.0.0.0:4000".parse()?;
    api_routes::init_server(addr).await?;

    Ok(())
}
