use iron::prelude::*;
use iron::middleware::{AfterMiddleware, BeforeMiddleware};

use db::DbConnection;

pub struct ErrorLog;

impl AfterMiddleware for ErrorLog {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let status = match res.status {
            Some(status) => format!("{}", status.to_u16()),
            None => "OK".to_owned(),
        };
        println!("{} {} {}", status, req.method, req.url);
        Ok(res)
    }

    fn catch(&self, req: &mut Request, err: IronError) -> IronResult<Response> {
        let status = match err.response.status {
            Some(status) => format!("{}", status.to_u16()),
            None => "ERROR".to_owned(),
        };
        println!("{} {} {}: {}", status, req.method, req.url, err);
        Err(err)
    }
}

impl BeforeMiddleware for DbConnection {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<DbConnection>(self.pool.clone());
        Ok(())
    }
}
