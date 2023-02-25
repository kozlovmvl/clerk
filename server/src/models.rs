use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct User {
    id: i32,
    username: String,
    token: String,
}
