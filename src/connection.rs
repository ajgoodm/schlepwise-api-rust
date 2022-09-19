use std::env;
use std::ops::Deref;

use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    Pool::new(manager).expect("db pool")
}

fn database_url() -> String {
    let host: String =
        env::var("SCHLEPWISE_POSTGRES_HOST").expect("SCHLEPWISE_POSTGRES_HOST must be set");
    let port: String =
        env::var("SCHLEPWISE_POSTGRES_PORT").expect("SCHLEPWISE_POSTGRES_PORT must be set");
    let user: String =
        env::var("SCHLEPWISE_POSTGRES_USER").expect("SCHLEPWISE_POSTGRES_USER must be set");
    let password: String =
        env::var("SCHLEPWISE_POSTGRES_PASSWORD").expect("SCHLEPWISE_POSTGRES_PASSWORD must be set");
    let db_name: String =
        env::var("SCHLEPWISE_POSTGRES_DB_NAME").expect("SCHLEPWISE_POSTGRES_DB_NAME must be set");

    format!(
        "postgres://{}:{}@{}:{}/{}",
        user, password, host, port, db_name
    )
}

pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
