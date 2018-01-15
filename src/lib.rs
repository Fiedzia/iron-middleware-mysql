extern crate iron;
extern crate mysql;

// insert database connection into request
use iron::{BeforeMiddleware, typemap};
use iron::prelude::{IronResult, Request};
use mysql::Pool;


pub struct DBPool {
    pub pool: Pool,
}

impl typemap::Key for DBPool {
    type Value = Pool;
}

impl BeforeMiddleware for DBPool {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<DBPool>(self.pool.clone());
        Ok(())
    }
}
