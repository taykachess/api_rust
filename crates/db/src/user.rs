use serde::{Deserialize, Serialize};
use surrealdb::{sql::Thing, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
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

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
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
        // ID of each user is the same as username ??
        let id: Record = crate::DB
            .create(("user", &self.username))
            .content(self)
            .await?;

        println!("{id:?}");
        Ok(())
    }

    pub async fn get_user(id: &str) -> Result<Option<User>> {
        let user: Option<User> = crate::DB.select(("user", id)).await?;
        Ok(user)
    }
}
