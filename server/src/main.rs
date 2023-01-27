use actix_web::{web, App, HttpServer};

use clerk::{http, ws};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::scope("/user")
            .service(http::login)
            .service(http::list_users)
            .service(http::get_user)
        ).service(ws::index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
