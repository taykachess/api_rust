#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr_db = "127.0.0.1:8000";
    let ns = "test";
    let db = "test";
    println!("before problem");
    api_db::surreal::init_db(addr_db, ns, db).await?;
    // api_db::prisma_client::init_bd().await;
    println!("after problem");

    let addr = "0.0.0.0:4000".parse()?;
    api_routes::init_server(addr).await?;

    Ok(())
}
