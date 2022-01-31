use std::env;
use std::ops::Deref;

use diesel::pg::PgConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome};
use rocket::Request;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    Pool::builder()
        .build(manager)
        .expect("Error building a connection pool")
}

fn database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

#[derive(Debug)]
pub enum PoolError {
    ConnectionRetrieval,
}

pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DbConn {
    type Error = PoolError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let pool = request.rocket().state::<Pool>().unwrap();
        // let outcome = request.guard::<&rocket::State<Pool>>().await;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => {
                Outcome::Failure((Status::ServiceUnavailable, PoolError::ConnectionRetrieval))
            }
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
