use serde::{Deserialize, Serialize};

use surrealdb::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub pass: String,
    info: UserInfo,
    roles: Vec<Role>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserId {
    id: i64,
    pub username: String,
    pub pass: String,
    info: UserInfo,
    roles: Vec<Role>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct UserInfo {
    name: String,
    surname: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum Role {
    Admin,
    Moderator,
    Author,
}

impl User {
    pub fn new(username: &str, pass: &str) -> Self {
        Self {
            username: username.to_owned(),
            pass: pass.to_owned(),
            info: Default::default(),
            roles: vec![],
        }
    }
    #[cfg(feature = "surreal")]
    pub async fn create_user(&self) -> Result<User> {
        let user = crate::surreal::DB.create("user").content(self).await?;
        println!("{:?}", user);
        Ok(user)
    }

    // #[cfg(feature = "prisma")]
    // pub async fn create_user(&self) -> Result<User> {
    //     let user = crate::surreal::DB.create("user").content(self).await?;
    //     println!("{:?}", user);
    //     Ok(user)
    // }

    #[cfg(feature = "surreal")]
    pub async fn get_user(username: &str) -> Result<Option<User>> {
        println!("{username}");
        let mut user = crate::surreal::DB
            .query(format!("SELECT * FROM user WHERE username = admin"))
            .await?;

        let user_new: Option<UserId> = user.take(1)?;
        println!("{:?}", user_new);
        let user: Option<User> = user.take(1)?;
        println!("{user:?}");
        Ok(user)
    }
}
