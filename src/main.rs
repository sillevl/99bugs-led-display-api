extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate iron;
extern crate router;
extern crate serde;
extern crate bodyparser;
extern crate image;
extern crate driver_99bugs_display;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use router::Router;
use std::io::Read;
use image::GenericImageView;
use driver_99bugs_display::Display;

mod http_parser;

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
    
    if let Err(err) = http_parser::check_content_type(request) {
        response.status = "error".to_string();
        response.message = err.to_string();
        return generate_response(response);
    }
    
    let mut content: [u8; BODY_BUFFER] = [0x00; BODY_BUFFER];
    request.body.read(&mut content).unwrap();

    // let image = ;
    match image::load_from_memory(&content) {
        Ok(image) => {
            let image = image.to_rgb();
            println!("dimensions {:?}", image.dimensions());

            let mut display = Display::new("/dev/spidev0.0");
            
            display.write_frame(&image);
            display.flush();
            println!("Done...");
        },
        Err(err) => {
            response.status = "error".to_string();
            response.message = err.to_string();
            return generate_response(response);
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