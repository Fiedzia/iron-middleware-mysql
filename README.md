# Mysql connection pool middleware for Iron web framework

This middleware will insert connection pool into request.extensions

Usage:

```rust

    extern crate iron;
    extern crate mysql;
    extern crate iron_middleware_mysql;
    
    use iron::prelude::*;
    use mysql::conn::pool::Pool;
    use iron_middleware_mysql::DBPool;
    
    
    fn hello_world(request: &mut Request) -> IronResult<Response> {
        let db_pool = request.extensions.get::<DBPool>().unwrap();
        let res = db_pool.prep_exec("select 23", ()).unwrap().next().unwrap().unwrap().unwrap()[0].clone();
        Ok(Response::with((iron::status::Ok, format!("Got {:?}", res))))
    }
    
    
    fn main() {
    
        let pool = Pool::new("mysql://user:password@localhost:3306").unwrap();
    
        let dbpool = DBPool{pool: pool.clone()};
        let mut chain = Chain::new(hello_world);
        chain.link_before(dbpool);
        Iron::new(chain).http("localhost:3000").unwrap();
    }


```

See included example for details
