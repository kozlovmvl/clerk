use chrono::{NaiveDateTime, offset::Utc};
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

use crate::db::PgConn;
use crate::schema::authtokens;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Queryable, Serialize)]
pub struct AuthToken {
    pub value: String,
    pub created: NaiveDateTime,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name=authtokens)]
pub struct NewAuthToken {
    pub value: String,
    pub created: NaiveDateTime,
    pub user_id: i32,
}

impl User {

    pub fn get(conn: &mut PgConn, username_: &str, password_: &str) -> QueryResult<Self>{
        use crate::schema::users::dsl::*;
        let query = users
            .filter(username.eq(username_).and(password.eq(password_)))
            .select((id, username))
            .first::<Self>(conn);
        query
    }
}

impl AuthToken {

    pub fn get_or_create(conn: &mut PgConn, user_id_: i32) -> String {
        use crate::schema::authtokens::dsl::*;
        let query = authtokens
            .filter(user_id.eq(user_id_))
            .select((value, created, user_id))
            .first::<Self>(conn);
        match query {
            Ok(token) => token.value.clone(),
            Err(_) => { 
                let authtoken = Self::create(conn, user_id_);
                authtoken.value
            }
        }
    }

    pub fn create(conn: &mut PgConn, user_id_: i32) -> Self {
        let new_authtoken = NewAuthToken{ 
            value: Uuid::new_v4().to_string(), 
            created: Utc::now().naive_utc(), 
            user_id: user_id_,
        };
        let created_authtoken = diesel::insert_into(authtokens::table)
            .values(&new_authtoken)
            .get_result(conn)
            .expect("Failed to create new authtoken.");
        created_authtoken
    }

    pub fn update(&mut self, conn: &mut PgConn) -> Self {
        use crate::schema::authtokens::dsl::*;
        let new_value = Uuid::new_v4().to_string();
        let new_created = Utc::now().naive_utc();
        let updated_authtoken = diesel::update(authtokens.find(self.value.clone()))
            .set((value.eq(new_value), created.eq(new_created)))
            .get_result::<AuthToken>(conn)
            .expect("Failed to update authtoken.");
        updated_authtoken
    }
}
