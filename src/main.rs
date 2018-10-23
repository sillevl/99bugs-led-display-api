#[macro_use]
extern crate serde_json;

extern crate iron;
extern crate router;
extern crate serde;
extern crate bodyparser;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use iron::request::*;
use router::Router;

fn handler(req: &mut Request) -> IronResult<Response> {

    let json_body = req.get::<bodyparser::Json>();
    match json_body {
        Ok(Some(json_body)) => {
            let image = json_body["image"].as_str().unwrap();
            let filename = json_body["filename"].as_str().unwrap();
            if(image.starts_with("data:image/png;base64,")) {
                println!("Is data URL");
            } else {
                println!("is NOT a data URL");
            }
        },
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {:?}", err)
    }

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