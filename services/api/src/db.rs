use crate::error::Result;
use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type PooledConn = PooledConnection<ConnectionManager<PgConnection>>;
pub type ConnPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_pool(database_url: &Option<String>) -> Result<ConnPool> {
    let database_url = match database_url {
        &Some(ref url) => url.to_owned(),
        &None => env::var("DATABASE_URL").expect("DATABASE_URL must be set."),
    };
    let manager = ConnectionManager::new(database_url.to_owned());

    Pool::new(manager).map_err(|err| err.into())
}
