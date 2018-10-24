#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate iron;
extern crate router;
extern crate serde;
extern crate bodyparser;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use iron::request::*;
use router::Router;
use serde_json::Error;

#[derive(Serialize, Deserialize)]
struct JsonResponse {
    status: String,
    message: String,
}

fn generate_response(response: JsonResponse) -> IronResult<Response> {
    let body = serde_json::to_string(&response).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, body)))
}

fn handler(req: &mut Request) -> IronResult<Response> {

    let mut response = JsonResponse {
        status: "Ok".to_owned(),
        message: "Request handled successfully".to_owned(),
    };

    let json_body = req.get::<bodyparser::Json>();
    match json_body {
        Ok(Some(json_body)) => {
            let image = json_body["image"].as_str().unwrap();
            let filename = json_body["filename"].as_str().unwrap();
            if image.starts_with("data:image/png;base64,") {
                println!("Is data URL");
            } else {
                println!("is NOT a data URL");
            }
        },
        Ok(None) => {
            println!("No body");
            response.status = "Error".to_owned();
            response.message = "Request has no body".to_owned();
        },
        Err(err) => {
            println!("Error: {:?}", err);
            response.status = "Error".to_owned();
            response.message = err.to_string();
        }
    }

    return generate_response(response);    
}

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "index");
    router.post("/", handler, "index");

    Iron::new(router).http("localhost:3000").unwrap();
}