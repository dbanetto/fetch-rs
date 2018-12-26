use crate::error::Result;
use crate::db::PooledConn;
use diesel::prelude::*;

// pub mod api;

// pub fn routes() -> Mount {
//     let mut mount = Mount::new();

//     // endpoints
//     mount.mount("/healthcheck", healthcheck);
//     mount.mount("/", api::routes());

//     mount
// }

pub fn healthcheck(conn: PooledConn) -> Result<bool> {
    (&*conn).execute("SELECT 1;")
        .map(|_| true)
        .map_err(|err| err.into())
}
