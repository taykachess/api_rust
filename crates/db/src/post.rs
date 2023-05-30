use serde::{Deserialize, Serialize};
use surrealdb::Result;
use uuid::Uuid;

use crate::user::Record;

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    // Get rid of ID
    username: String,
    title: String,
    body: String,
}

impl Post {
    pub fn new(username: &str, title: &str, body: &str) -> Self {
        Self {
            username: username.to_owned(),
            title: title.to_owned(),
            body: body.to_owned(),
        }
    }

    pub async fn create_post(self) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        // TODO: move Record type to separate rs file
        let _: Record = crate::DB.create(("post", &id)).content(self).await?;

        Ok(id)
    }

    pub async fn update_post(self, id: Uuid) -> Result<()> {
        let _: Option<Post> = crate::DB
            .update(("post", id.to_string()))
            .content(self)
            .await?;

        Ok(())
    }

    pub async fn get_all_post() -> Result<Vec<Post>> {
        let posts: Vec<Post> = crate::DB.select("post").await?;

        Ok(posts)
    }
}
