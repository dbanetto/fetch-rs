use std::env;

use iron::typemap;
use dotenv::dotenv;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use r2d2::{Pool, Config};

pub struct DbConnection {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl typemap::Key for DbConnection { type Value = Pool<ConnectionManager<PgConnection>>; }

pub fn get_pool() -> DbConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let config = Config::default();
    let manager = ConnectionManager::new(database_url);
    let pool = Pool::new(config, manager).expect("Failed to create pool.");

    DbConnection { pool: pool }
}
