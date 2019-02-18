extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate iron;
extern crate router;
extern crate serde;
extern crate bodyparser;
extern crate image;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use router::Router;
use std::io::Read;
use image::GenericImageView;

mod http_parser;
mod png_parser;

pub const BODY_BUFFER: usize = 1024;

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

fn handler(request: &mut Request) -> IronResult<Response> {

    let mut response = JsonResponse {
        status: "Ok".to_owned(),
        message: "Request handled successfully".to_owned(),
    };
    
    http_parser::check_content_type(request).expect("Content-type is set to image/png");
    http_parser::check_content_size(request).expect("Content-size is set");
    
    let mut content: [u8; BODY_BUFFER] = [0x00; BODY_BUFFER];
    request.body.read(&mut content).unwrap();
    png_parser::check_png_header(&content).expect("PNG header to be found in content");

    let img = image::load_from_memory(&content).unwrap().to_rgb();
    println!("dimensions {:?}", img.dimensions());

    // match content_type {
    //     Ok(Some(body)) => println!("Read body:\n{}", body),
    //     Ok(None) => println!("No body"),
    //     Err(err) => println!("Error: {:?}", err)
    // }

    return generate_response(response);    
}

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "index");
    router.post("/", handler, "index");

    Iron::new(router).http("localhost:3000").unwrap();
}