extern crate chrono;
extern crate env_logger;
extern crate iron;
extern crate logger;
extern crate router;
extern crate serde;
extern crate uuid;




mod models;
mod database;
mod handler;

use models::*;
use database::Database;
use handler::*;

use iron::prelude::Chain;
use iron::Iron;
use router::Router;
use logger::Logger;
use uuid::Uuid;


fn main() {
    env_logger::init().unwrap();
    // let (logger_before, logger_after) = Logger::new(None);

    let mut db = Database::new();
    let p1 = Post::new(
        "first",
        "post first",
        "mike",
        chrono::offset::Utc::now(),
        Uuid::new_v4(),
    );

    let p2 = Post::new(
        "second",
        "post second",
        "mike",
        chrono::offset::Utc::now(),
        Uuid::new_v4(),
    );

    db.add_post(p1);
    db.add_post(p2);

    let handlers = Handlers::new(db);
    let jmw = JsonAfterMiddleware;

    let mut router = Router::new();
    router.get("/post_feed", handlers.post_feed, "post_feed");
    router.post("/post", handlers.post_post, "post_post");
    router.get("/post/:id", handlers.post, "post");

    let mut chain = Chain::new(router);
    // chain.link_before(logger_before);
    chain.link_after(jmw);
    // chain.link_after(logger_after);

    Iron::new(chain).http("localhost:8080").unwrap();
}
