use api_routes::auth::UserDto;
use surrealdb::engine::local::Db;
use surrealdb::engine::local::Mem;
use surrealdb::{Result, Surreal};

// static DB: Surreal<Db> = Surreal::init();

// pub async fn init_db(ns: &str, db: &str) -> Result<()> {
//     DB.connect::<Mem>(()).await?;
//     DB.use_ns(ns).use_db(db).await?;

//     Ok(())
// }

#[tokio::main]
async fn main() {
    // test1()
    println!("Main")
}

#[tokio::test]
async fn test1() -> anyhow::Result<()> {
    api_db::init_db("0.0.0.127", "test", "test").await?;
    let user = api_routes::auth::signup(axum::Json(UserDto::new("username", "pass"))).await;

    match user {
        Ok(user) => println!("User, where are you {:?}", user),
        Err(e) => println!("Error where are you {:?}", e),
    }
    println!("test is over");
    println!("Nice to have to play");
    Ok(())
}
