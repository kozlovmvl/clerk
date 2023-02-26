#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use diesel::prelude::*;

use clerk::db::{get_pool, DbConn};
use clerk::models::User;
use clerk::json::*;

#[post("/login", data="<request>")]
async fn login(mut conn: DbConn, request: Json<LoginRequest>) -> Json<LoginResponse> {
    use clerk::schema::users::dsl::*;
    let query = users
        .filter(username.eq(request.username.clone()).and(password.eq(request.password.clone())))
        .select((id, username, token))
        .first::<User>(&mut conn.0);
    match query {
        Ok(user) => Json(LoginResponse{ token: user.token.clone() }),
        Err(_) => Json(LoginResponse{ token: "".to_string() })
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .manage(get_pool())
        .mount("/auth", routes![login])
        .launch()
        .await?;
    Ok(())
}
