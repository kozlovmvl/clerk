use std::ops::Deref;
use diesel::r2d2::{Pool, PooledConnection, ConnectionManager};
use diesel::pg::PgConnection;

use rocket::http::Status;
use rocket::request::{Request, FromRequest, Outcome};

type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgConn = PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_pool() -> PgPool {
    let url = "postgres://admin:password@127.0.0.1:5432/clerk_db";
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Failed to create pool.")
}

pub struct DbConn(pub PgConn);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DbConn {
    type Error = ();
    
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let pool = request.rocket().state::<PgPool>().unwrap();
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
