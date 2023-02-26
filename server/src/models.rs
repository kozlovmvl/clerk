use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub token: String,
}
