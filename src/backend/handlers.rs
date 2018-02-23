use std::path::PathBuf;

use iron::prelude::*;
use time::{self, Timespec};
use std::time::Duration;
use iron::{status, Handler};
use std::fs;

pub struct CachedFile {
    path: PathBuf,
    timeout: Duration,
}


impl CachedFile {
    pub fn new<P: Into<PathBuf>>(path: P, timeout: Duration) -> Self {
        CachedFile {
            path: path.into(),
            timeout: timeout,
        }
    }

    fn response_with_cache(
        &self,
        req: &mut Request,
        size: u64,
        modified: Timespec,
    ) -> IronResult<Response> {
        use iron::headers::{CacheControl, CacheDirective, HttpDate, LastModified};
        use iron::headers::{ContentLength, ContentType, ETag, EntityTag};
        use iron::method::Method;
        use iron::mime::{Mime, SubLevel, TopLevel};
        use iron::modifiers::Header;

        let seconds = self.timeout.as_secs() as u32;
        let cache = vec![CacheDirective::Public, CacheDirective::MaxAge(seconds)];
        let metadata = fs::metadata(self.path.clone());

        let metadata = try!(metadata.map_err(|e| IronError::new(e, status::InternalServerError)));

        let mut response = if req.method == Method::Head {
            let has_ct = req.headers.get::<ContentType>();
            let cont_type = match has_ct {
                None => ContentType(Mime(TopLevel::Text, SubLevel::Plain, vec![])),
                Some(t) => t.clone(),
            };
            Response::with((
                status::Ok,
                Header(cont_type),
                Header(ContentLength(metadata.len())),
            ))
        } else {
            Response::with((status::Ok, self.path.clone()))
        };

        response.headers.set(CacheControl(cache));
        response
            .headers
            .set(LastModified(HttpDate(time::at(modified))));
        response.headers.set(ETag(EntityTag::weak(format!(
            "{0:x}-{1:x}.{2:x}",
            size,
            modified.sec,
            modified.nsec
        ))));

        Ok(response)
    }
}

impl Handler for CachedFile {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        use iron::headers::{HttpDate, IfModifiedSince};

        let (size, last_modified_time) = match fs::metadata(&self.path) {
            Err(error) => return Err(IronError::new(error, status::InternalServerError)),
            Ok(metadata) => {
                use filetime::FileTime;

                let time = FileTime::from_last_modification_time(&metadata);
                (metadata.len(), Timespec::new(time.seconds() as i64, 0))
            }
        };

        let if_modified_since = match req.headers.get::<IfModifiedSince>().cloned() {
            None => return self.response_with_cache(req, size, last_modified_time),
            Some(IfModifiedSince(HttpDate(time))) => time.to_timespec(),
        };

        if last_modified_time <= if_modified_since {
            Ok(Response::with(status::NotModified))
        } else {
            self.response_with_cache(req, size, last_modified_time)
        }
    }
}
