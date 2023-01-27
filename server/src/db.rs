use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn establish_connection() -> PgConnection {
    let database_url = "postgres://admin:password@127.0.0.1:5432/clerk_db".to_string();
    PgConnection::establish(&database_url).unwrap()
}
