use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Result, Surreal,
};

pub static DB: Surreal<Client> = Surreal::init();

pub async fn init_db(addr: &str, ns: &str, db: &str) -> Result<()> {
    DB.connect::<Ws>(addr).await?;
    // Select a namespace + database
    DB.use_ns(ns).use_db(db).await?;

    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    Ok(())
}
