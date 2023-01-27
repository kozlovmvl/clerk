use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct OkLogin {
    pub token: String,
}

#[derive(Serialize)]
pub struct ErrLogin {
    pub value: String,
}
