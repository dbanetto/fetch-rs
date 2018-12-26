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
// pub mod util;

use crate::error::{Error, Result};
use dotenv::dotenv;
use std::error::Error as StdError;

use warp::Filter;

use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

fn api_response<T: serde::Serialize>(result: Result<T>) -> impl warp::Reply {
    let mut value = std::collections::HashMap::new();

    match result {
        Ok(data) => {
            value.insert("success", serde_json::json!(true));
            value.insert("data", serde_json::to_value(data).unwrap());
        }
        Err(err) => {
            value.insert("success", serde_json::json!(false));
            value.insert("error", serde_json::to_value(err.description()).unwrap());
        }
    };

    warp::reply::json(&value)
}

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
    let db_pool = db::get_pool(&config.database_url)?;

    let db_conn = warp::any().and_then(move || {
        db_pool
            .get()
            .map_err(|err| warp::reject::custom(err.description()))
    });

    let healthcheck = warp::filters::method::get2()
            .and(path!("healthcheck"))
            .and(db_conn.clone())
            .map(routes::healthcheck)
            .map(api_response);
                    

    let default = warp::any()
        .and(db_conn.clone())
        .map(|pool: db::PooledConn| models::Series::all(&*pool))
        .map(api_response);

    let select = warp::filters::method::get2()
        .and(db_conn.clone())
        .and(path!("api" / "series" / i32))
        .map(|pool: db::PooledConn, id: i32| models::Series::get(&*pool, id))
        .map(api_response);

    let routes = warp::any()
        .and(healthcheck
             .or(select)
             .or(default))
        .with(warp::log("api"));

    println!("Starting server on {}", addr);
    warp::serve(routes).run(addr);

    Ok(())
}
