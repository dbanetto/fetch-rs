use iron::prelude::*;
use iron::middleware::{AfterMiddleware, BeforeMiddleware};
use iron::typemap;
use std::time::Instant;
use hbs::HandlebarsEngine;
use hbs::handlebars::Handlebars;
use serde_json::Value;
use durationfmt;

use config::Config;
use db::DbConnection;

pub struct ErrorLog;

impl AfterMiddleware for ErrorLog {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let duration = match req.extensions.get::<Timer>() {
            Some(timer) => timer.elapsed(),
            None => return Ok(res),
        };

        let status = match res.status {
            Some(status) => format!("{}", status.to_u16()),
            None => "OK".to_owned(),
        };
        println!(
            "{} {} {} ({})",
            status,
            req.method,
            req.url,
            // This duration does not include time spent writing to net
            // only the time to route & create a response
            durationfmt::to_string(duration)
        );
        Ok(res)
    }

    fn catch(&self, req: &mut Request, err: IronError) -> IronResult<Response> {
        let duration = match req.extensions.get::<Timer>() {
            Some(timer) => timer.elapsed(),
            None => return Err(err),
        };

        let status = match err.response.status {
            Some(status) => format!("{}", status.to_u16()),
            None => "ERROR".to_owned(),
        };
        println!(
            "{} {} {}: {} ({})",
            status,
            req.method,
            req.url,
            err,
            // This duration does not include time spent writing to net
            // only the time to route & create a response
            durationfmt::to_string(duration)
        );
        Err(err)
    }
}

impl BeforeMiddleware for DbConnection {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<DbConnection>(self.pool.clone());
        Ok(())
    }
}

impl typemap::Key for Config {
    type Value = Value;
}

impl BeforeMiddleware for Config {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<Config>(self.template_config());
        Ok(())
    }
}

pub struct Timer;

impl BeforeMiddleware for Timer {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<Timer>(Instant::now());
        Ok(())
    }
}

impl typemap::Key for Timer {
    type Value = Instant;
}

pub fn handlebars() -> HandlebarsEngine {
    let mut hb = Handlebars::new();

    hb.register_template_string("index", include_str!("templates/index.hbs"))
        .unwrap();

    HandlebarsEngine::from(hb)
}
