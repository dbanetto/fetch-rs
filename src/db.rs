use std::env;

use iron::typemap;
use dotenv::dotenv;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

pub struct DbConnection {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl typemap::Key for DbConnection {
    type Value = Pool<ConnectionManager<PgConnection>>;
}

pub fn get_pool(database_url: Option<String>) -> DbConnection {
    let database_url = match database_url {
        Some(url) => url,
        None => env::var("DATABASE_URL").expect("DATABASE_URL must be set."),
    };
    let manager = ConnectionManager::new(database_url);
    let pool = Pool::new(manager).expect("Failed to create pool.");

    DbConnection { pool: pool }
}
