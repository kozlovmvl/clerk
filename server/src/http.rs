use actix_web::{get, post, web, HttpResponse, Responder, Result};
use http::StatusCode;

use crate::services::{user_login, get_list_users, get_one_user};

#[post("/login")]
async fn login(request: String) -> Result<impl Responder> {
    match user_login(&request) {
        Ok(success) => Ok(HttpResponse::Ok().json(success)),
        Err(err) => Ok(HttpResponse::build(StatusCode::BAD_REQUEST).json(err))
    }
}

#[get("/list")]
async fn list_users() -> Result<impl Responder> {
    let users = get_list_users();
    Ok(HttpResponse::Ok().json(users))
}

#[get("/{id}")]
async fn get_user(id: web::Path<(i32,)>) -> Result<impl Responder> {
    let user = get_one_user(id.0);
    Ok(HttpResponse::Ok().json(user))
}
