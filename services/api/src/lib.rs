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
