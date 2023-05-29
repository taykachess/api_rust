use surrealdb::engine::local::Db;
use surrealdb::engine::local::Mem;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    Result, Surreal,
};

static DB: Surreal<Db> = Surreal::init();

#[cfg(test)]
pub async fn init_db(ns: &str, db: &str) -> Result<()> {
    println!("TEST");
    DB.connect::<Mem>(()).await?;
    // Select a namespace + database
    DB.use_ns(ns).use_db(db).await?;

    Ok(())
}

#[cfg(not(test))]
pub async fn init_db(addr: &str, ns: &str, db: &str) -> Result<()> {
    println!("PRODUCTION");
    DB.connect::<Mem>(()).await?;
    // Select a namespace + database
    DB.use_ns(ns).use_db(db).await?;

    // https://github.com/surrealdb/surrealdb/issues/1949
    Ok(())
}

pub mod post;
pub mod user;
