use surrealdb::{
    engine::remote::ws::{Client, Ws},
    Result, Surreal,
};

static DB: Surreal<Client> = Surreal::init();

pub async fn init_db(addr: &str, ns: &str, db: &str) -> Result<()> {
    DB.connect::<Ws>(addr).await?;
    // Select a namespace + database
    DB.use_ns(ns).use_db(db).await?;

    Ok(())
}

pub mod post;
pub mod user;
