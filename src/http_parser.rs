
extern crate iron;
extern crate router;
extern crate serde;
extern crate bodyparser;

use iron::prelude::*;

use iron::headers::{ ContentType };

pub fn check_content_type(request: &Request) -> Result<(), String> {
    let content_type = request.headers.get::<ContentType>().unwrap();
    if content_type != &ContentType::png() {
        // println!("content type is not PNG");
        return Err("content type is not PNG".to_string());
    }
    Ok(())
}

pub fn check_content_size(request: &Request) -> Result<(), String> {
    // TODO: implement
    Ok(())
}