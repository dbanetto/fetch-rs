use std::env;

use dotenv::dotenv;
use diesel::pg::PgConnection;
use diesel::connection::Connection;
use r2d2_diesel::ConnectionManager;
use r2d2::{Pool, Config, PooledConnection, GetTimeout};

lazy_static! {
    pub static ref DB_POOL: Pool<ConnectionManager<PgConnection>> = {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let config = Config::default();
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::new(config, manager).expect("Failed to create pool.");
        pool
    };
}

pub struct DB(PooledConnection<ConnectionManager<PgConnection>>);

impl DB {
    pub fn conn(&self) -> &PgConnection {
        &*self.0
    }
}
