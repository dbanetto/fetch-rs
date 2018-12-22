use iron::middleware::{AfterMiddleware, BeforeMiddleware};
use iron::prelude::*;
use iron::typemap;
use std::time::Instant;

use crate::db::DbConnection;

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

        let duration_ms =
            (duration.as_secs() as f64 * 1_000.0) + (duration.subsec_nanos() as f64 * 1e-6);

        println!(
            "{} {} {} ({:.5}ms)",
            status,
            req.method,
            req.url.as_ref().path(),
            // This duration does not include time spent writing to net
            // only the time to route & create a response
            duration_ms
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

        let duration_ms =
            (duration.as_secs() as f64 * 1_000.0) + (duration.subsec_nanos() as f64 * 1e-6);

        println!(
            "{} {} {}: {} ({:.5}ms)",
            status,
            req.method,
            req.url.as_ref().path(),
            err,
            // This duration does not include time spent writing to net
            // only the time to route & create a response
            duration_ms
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
