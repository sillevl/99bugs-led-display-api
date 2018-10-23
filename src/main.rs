#[macro_use]
extern crate serde_json;

extern crate iron;
extern crate router;
extern crate serde;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use router::Router;

fn handler(req: &mut Request) -> IronResult<Response> {

    let response = json!({"status": "OK"});
    let body = serde_json::to_string(&response).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, body)))
}

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "index");
    router.post("/", handler, "index");

    Iron::new(router).http("localhost:3000").unwrap();
}