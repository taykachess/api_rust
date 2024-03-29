use surrealdb::engine::local::Db;
use surrealdb::engine::local::Mem;
use surrealdb::{Result, Surreal};

static DB: Surreal<Db> = Surreal::init();

/// **Initialize DB**
/// This function only for testing
/// ``` init_db_test() ```
pub async fn init_db_test() -> Result<()> {
    DB.connect::<Mem>(()).await?;
    // Select a namespace + database
    let db = "test";
    DB.use_ns(db).use_db(db).await?;
    Ok(())
}

pub async fn init_db(ns: &str, db: &str) -> Result<()> {
    println!("PRODUCTION");
    DB.connect::<Mem>(()).await?;
    DB.use_ns(ns).use_db(db).await?;

    // https://github.com/surrealdb/surrealdb/issues/1949
    Ok(())
}

pub mod post;
pub mod user;
