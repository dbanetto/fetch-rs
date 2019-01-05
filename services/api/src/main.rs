#![recursion_limit = "256"]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derives;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate warp;

pub mod config;
pub mod db;
pub mod error;
pub mod models;
pub mod routes;
pub mod schema;
pub mod util;

use crate::error::{Error, Result};
use dotenv::dotenv;

use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = match config::get_config() {
        Ok(config) => config,
        Err(err) => {
            log::error!("Config error: {}", err);
            std::process::exit(1);
        }
    };

    let ip = IpAddr::from_str(&config.bind).map_err::<Error, _>(|err| err.into())?;
    let addr = SocketAddr::from((ip, config.port));
    let db_conn = db::get_pool(&config.database_url)?;

    println!("Starting server on {}", addr);
    warp::serve(routes::routes(db_conn)).run(addr);

    Ok(())
}
