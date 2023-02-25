#[macro_use] extern crate rocket;
use rocket::serde::json::Json;

use clerk::json::*;

#[post("/login", data="<request>")]
async fn login(request: Json<LoginRequest>) -> Json<LoginResponse> {
    Json(LoginResponse{ token: "token".to_string() })
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/auth", routes![login])
        .launch()
        .await?;
    Ok(())
}
