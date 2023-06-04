use serde::{Deserialize, Serialize};
use surrealdb::{sql::Thing, Result};

use chrono::{DateTime, Duration, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthUser {
    pub username: String,
    pub roles: Vec<Role>,
    exp: usize,
}

impl AuthUser {
    pub fn new(username: &str) -> Self {
        let now: DateTime<Utc> = Utc::now();
        let one_week_later: DateTime<Utc> = now + Duration::weeks(1);
        let one_week_later = one_week_later.timestamp() as usize;
        Self {
            username: username.to_owned(),
            roles: vec![],
            // TODO UTC timestamp
            exp: one_week_later,
        }
    }
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Role {
    Admin,
    Moderator,
    Author,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}

impl Record {
    pub fn id(&self) -> &Thing {
        &self.id
    }
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

        Ok(())
    }

    pub async fn get_user(id: &str) -> Result<Option<User>> {
        let user: Option<User> = crate::DB.select(("user", id)).await?;
        Ok(user)
    }
}
