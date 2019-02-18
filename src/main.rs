extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate iron;
extern crate router;
extern crate serde;
extern crate bodyparser;
extern crate base64;

mod parser;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use router::Router;
use base64::decode;

#[derive(Serialize, Deserialize)]
struct JsonResponse {
    status: String,
    message: String,
}

fn get_json_from_request(request: &mut Request) -> Result<serde_json::Value, String> {
    let json_body = request.get::<bodyparser::Json>();
    match json_body {
        Ok(Some(json_body)) => {
            return Ok(json_body["image"].clone());
            // if image.starts_with("data:image/png;base64,") {
            //     println!("Is data URL");
            // } else {
            //     println!("is NOT a data URL");
            // }
        },
        Ok(None) => {
            println!("No body");
            return Err(String::from("Request has no body"));
        },
        Err(err) => {
            println!("Error: {:?}", err);
            return Err(err.to_string())
        }
    }
}

// fn get_data_from_body(request: &Request) -> String {

// }

fn generate_response(result: Result<(), String>) -> IronResult<Response> {
    let mut response = JsonResponse {
        status: "Ok".to_owned(),
        message: "Request handled successfully".to_owned(),
    };

    let mut status_code = status::Ok;

    match result {
        Ok(_) => {

        },
        Err(err) => {
            status_code = status::BadRequest;
            response.status = String::from("Error");
            response.message = err.to_string();
        }
    }

    let body = serde_json::to_string(&response).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status_code, body)))
}

fn data_from_png(base64_data: &String) {
    let data = base64::decode(&base64_data).unwrap();
}

fn write_data_to_display(data: &[u8]) {

}

fn get_data_url_data(data_url: &String) {

}

fn get_data_url_format(data_url: &String) {
    
}

fn handler(request: &mut Request) -> IronResult<Response> {
    let json = get_json_from_request(request).unwrap();
    let imgage_url = &json["image"].to_string();
    let base64_data = get_data_url_data(&imgage_url);
    let image_format = get_data_url_format(&imgage_url);
    let data = match image_format {
        "png" => data_from_png(&data),
    };
    write_data_to_display(&data);
    


    // let good_result: Result<(), &str> = Ok(());
    // let bad_result: Result<(), &str> = Err("Something went wrong");

    return generate_response(Ok(()));    
}

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "index");
    router.post("/", handler, "index");

    Iron::new(router).http("localhost:3000").unwrap();
}