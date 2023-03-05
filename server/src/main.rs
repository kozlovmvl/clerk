#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rocket::http::Status;

use clerk::db::{get_pool, DbConn};
use clerk::models::{User, AuthToken};
use clerk::json::*;

#[post("/login", data="<request>")]
async fn login(mut conn: DbConn, request: Json<LoginRequest>) -> Result<Json<LoginResponse>, Status> {
    let user: User = match User::get(&mut conn.0, &request.username, &request.password) {
        Ok(value) => value,
        Err(_) => return Err(Status::Unauthorized)
    };
    let token: String = AuthToken::get(&mut conn.0, user.id);
    Ok(Json(LoginResponse{ token: token }))
}

#[post("/logout")]
async fn logout(user: User) -> &'static str {
    println!("{}", user.username);
    "logout"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .manage(get_pool())
        .mount("/auth", routes![login, logout])
        .launch()
        .await?;
    Ok(())
}
