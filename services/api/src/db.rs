use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use iron::typemap;
use r2d2::Pool;

pub struct DbConnection {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl typemap::Key for DbConnection {
    type Value = Pool<ConnectionManager<PgConnection>>;
}

pub fn get_pool(database_url: &Option<String>) -> DbConnection {
    let database_url = match database_url {
        &Some(ref url) => url.to_owned(),
        &None => env::var("DATABASE_URL").expect("DATABASE_URL must be set."),
    };
    let manager = ConnectionManager::new(database_url.to_owned());
    let pool = Pool::new(manager).expect("Failed to create pool.");

    DbConnection { pool: pool }
}
