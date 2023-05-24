use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    id: Uuid,
    username: String,
    title: String,
    body: String,
}
