use actix_web::{error::ErrorServiceUnavailable, Error, FromRequest};
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use futures::future::{err, ok, Ready};
use lazy_static::lazy_static;

use crate::config::Config;

type PgPool = Pool<ConnectionManager<PgConnection>>;
pub struct DbConnection {
    pub db_connection: PgPool,
}

lazy_static! {
    pub static ref DB_CONNECTION: DbConnection = {
        let Config {
            db_url: connection_string,
            ..
        } = Config::new();
        DbConnection {
            db_connection: PgPool::builder()
                .max_size(8)
                .build(ConnectionManager::new(connection_string))
                .unwrap(),
        }
    };
}

type PooledPgConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub struct DB {
    pub connection: PooledPgConnection,
}

impl FromRequest for DB {
    type Error = Error;

    type Future = Ready<Result<DB, Error>>;

    fn from_request(_: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        match DB_CONNECTION.db_connection.get() {
            Ok(connection) => ok(DB { connection }),
            Err(_) => err(ErrorServiceUnavailable("could not connect to database")),
        }
    }
}
