use crate::error::{Error, Result};
use std::env;
use std::error::Error as StdError;

use warp::Filter;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type PooledConn = PooledConnection<ConnectionManager<PgConnection>>;
pub type PooledConnFilter = warp::filters::BoxedFilter<(PooledConn,)>;

pub fn get_pool(database_url: &Option<String>) -> Result<PooledConnFilter> {
    let database_url = match database_url {
        &Some(ref url) => url.to_owned(),
        &None => env::var("DATABASE_URL").expect("DATABASE_URL must be set."),
    };
    let manager = ConnectionManager::new(database_url.to_owned());

    let pool = Pool::new(manager).map_err::<Error, _>(|err| err.into())?;

    Ok(warp::any()
        .and_then(move || {
            pool.get()
                .map_err(|err| warp::reject::custom(err.description()))
        })
        .boxed())
}
