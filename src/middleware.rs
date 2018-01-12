use iron::prelude::*;
use iron::middleware::{AfterMiddleware, BeforeMiddleware};
use iron::typemap;
use std::time::Instant;
use durationfmt;

use db::DbConnection;


pub struct ErrorLog;

impl AfterMiddleware for ErrorLog {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let duration = req.extensions.get::<Timer>().unwrap().elapsed();
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
        let duration = req.extensions.get::<Timer>().unwrap().elapsed();
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
