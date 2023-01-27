use diesel::prelude::*;
use serde_json;

use crate::db::establish_connection;
use crate::models::{User, LoginData};
use crate::schema::users::dsl::*;

pub fn user_login(data: &str) -> User {
    let mut conn = establish_connection();
    let login_data: LoginData = serde_json::from_str(data).expect("Error parse login data.");
    let result = users
        .filter(username.eq(login_data.username).and(password.eq(login_data.password)))
        .select((id, username))
        .first::<User>(&mut conn)
        .expect("Error login.");
    result
}

pub fn get_list_users() -> Vec<User> {
    let mut conn = establish_connection();
    let result = users
        .select((id, username))
        .load::<User>(&mut conn)
        .expect("Error loading users.");
    result
}

pub fn get_one_user(_id: i32) -> User {
    let mut conn = establish_connection();
    let result = users
        .filter(id.eq(_id))
        .select((id, username))
        .first::<User>(&mut conn)
        .expect("Error loading user.");
    result
}
