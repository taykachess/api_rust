use serde::{Deserialize, Serialize};
use surrealdb::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    username: String,
    pass: String,
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

    pub async fn create_user(&self) -> Result<()> {
        crate::DB.create("user").content(self).await?;
        Ok(())
    }

    pub async fn get_user(id: &str) -> Result<User> {
        let user: User = crate::DB.select(("user", id)).await?;
        Ok(user)
    }
}
