use iron::prelude::*;
use iron::middleware::{BeforeMiddleware, AfterMiddleware};

use ::db::DbConnection;

pub struct ErrorLog;


impl AfterMiddleware for ErrorLog {
    fn catch(&self, req: &mut Request, err: IronError) -> IronResult<Response> {
        println!("ERROR {}: {}", req.url, err);
        Err(err)
    }
}

impl BeforeMiddleware for DbConnection {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<DbConnection>(self.pool.clone());
        Ok(())
    }
}
