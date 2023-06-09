use serde::{Deserialize, Serialize};
use surrealdb::Result;
use uuid::Uuid;

use crate::user::Record;

#[derive(Deserialize, Serialize)]
pub struct PostCreateDto {
    // username: String,
    title: Option<String>,
    body: Option<String>,
}

impl PostCreateDto {
    pub fn new(title: Option<String>, body: Option<String>) -> Self {
        Self { title, body }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    // Get rid of ID
    username: String,
    title: Option<String>,
    body: Option<String>,
}

impl Post {
    pub fn new(
        PostCreateDto {
            // username,
            title,
            body,
        }: PostCreateDto,
        username: String,
    ) -> Self {
        Self {
            username,
            title,
            body,
        }
    }

    pub async fn create_post(self) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        // TODO: move Record type to separate rs file
        let _: Record = crate::DB.create(("post", &id)).content(self).await?;

        Ok(id)
    }

    pub async fn update_post(self, id: Uuid) -> Result<String> {
        let post: Record = crate::DB
            .update(("post", id.to_string()))
            .content(self)
            .await?;

        Ok(post.id().to_string())
    }

    pub async fn delete_post(id: Uuid) -> Result<String> {
        let post: Record = crate::DB.delete(("post", id.to_string())).await?;

        Ok(post.id().to_string())
    }

    pub async fn get_post(id: Uuid) -> Result<Post> {
        let post: Post = crate::DB.select(("post", id.to_string())).await?;

        Ok(post)
    }
}
