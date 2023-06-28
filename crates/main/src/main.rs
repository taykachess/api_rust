mod logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init().expect("Failed to init logger");
    dotenv::dotenv().ok();
    let ns = "test";
    let db = "test";
    api_db::init_db(ns, db).await?;

    let addr = "0.0.0.0:4000".parse()?;
    api_routes::init_server(addr).await?;

    Ok(())
}
