use chrono::{NaiveDateTime, offset::Utc};
use diesel::prelude::*;
use rocket::http::Status;
use rocket::request::{Request, FromRequest, Outcome};
use serde::Serialize;
use uuid::Uuid;

use crate::db::{PgConn, PgPool};
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

    pub fn get_by_id(conn: &mut PgConn, id_: i32) -> QueryResult<Self> {
        use crate::schema::users::dsl::*;
        let query = users
            .filter(id.eq(id_))
            .select((id, username))
            .first::<Self>(conn);
        query
    }
}

impl AuthToken {

    pub fn get(conn: &mut PgConn, user_id_: i32) -> String {
        use crate::schema::authtokens::dsl::*;
        let query = authtokens
            .filter(user_id.eq(user_id_))
            .select((value, created, user_id))
            .first::<Self>(conn);
        match query {
            Ok(mut authtoken) => {
                authtoken.update(conn);
                authtoken.value
            },
            Err(_) => { 
                let authtoken = Self::create(conn, user_id_);
                authtoken.value
            }
        }
    }

    pub fn get_by_value(conn: &mut PgConn, value_: &str) -> QueryResult<Self> {
        use crate::schema::authtokens::dsl::*;
        let query = authtokens
            .filter(value.eq(value_))
            .select((value, created, user_id))
            .first::<Self>(conn);
        query
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

    pub fn update(&mut self, conn: &mut PgConn) {
        use crate::schema::authtokens::dsl::*;
        let new_value = Uuid::new_v4().to_string();
        let new_created = Utc::now().naive_utc();
        diesel::update(authtokens.find(self.value.clone()))
            .set((value.eq(new_value.clone()), created.eq(new_created.clone())))
            .get_result::<AuthToken>(conn)
            .expect("Failed to update authtoken.");
        self.value = new_value;
        self.created = new_created;
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = req.headers().get_one("Authorization");
        let pool = req.rocket().state::<PgPool>().unwrap();
        let mut conn = match pool.get() {
            Ok(v) => v,
            Err(_) => return Outcome::Failure((Status::NonAuthoritativeInformation, ()))
        };
        match auth_header {
            Some(value) => {
                let authtoken = match AuthToken::get_by_value(&mut conn, value) {
                    Ok(v) => v,
                    Err(_) => return Outcome::Failure((Status::NonAuthoritativeInformation, ()))
                };
                let user = match Self::get_by_id(&mut conn, authtoken.user_id) {
                    Ok(v) => v,
                    Err(_) => return Outcome::Failure((Status::NonAuthoritativeInformation, ()))
                };
                Outcome::Success(user)
            },
            None => Outcome::Failure((Status::NonAuthoritativeInformation, ()))
        }
    }
}
