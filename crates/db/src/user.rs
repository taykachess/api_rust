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
    pub fn new(username: String, pass: String) -> Self {
        Self {
            username,
            pass,
            info: Default::default(),
            roles: vec![],
        }
    }

    pub async fn create_user(&self) -> Result<()> {
        crate::DB.create("user").content(self).await?;
        Ok(())
    }
}
