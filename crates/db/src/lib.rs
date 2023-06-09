use surrealdb::engine::local::Db;
use surrealdb::engine::local::Mem;
use surrealdb::{Result, Surreal};

static DB: Surreal<Db> = Surreal::init();

pub async fn init_db_test() -> Result<()> {
    DB.connect::<Mem>(()).await?;
    // Select a namespace + database
    let db = "test";
    DB.use_ns(db).use_db(db).await?;
    Ok(())

    // static INIT: OnceCell<Result<()>> = OnceCell::const_new();
    // let res = INIT
    // .get_or_init(|| async {
    //         DB.connect::<Mem>(()).await?;
    //         // Select a namespace + database
    //         let db = "test";
    //         DB.use_ns(db).use_db(db).await?;
    //         Ok(())
    //     })
    //     .await;

    // match res {
    //     Ok(_) => Ok(()),
    //     Err(e) => panic!("Problem with DB: {e}"),
    // }
}

pub async fn init_db(ns: &str, db: &str) -> Result<()> {
    println!("PRODUCTION");
    DB.connect::<Mem>(()).await?;
    // Select a namespace + database
    DB.use_ns(ns).use_db(db).await?;

    // https://github.com/surrealdb/surrealdb/issues/1949
    Ok(())
}

pub mod post;
pub mod user;
