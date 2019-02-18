
extern crate iron;
extern crate router;
extern crate serde;
extern crate bodyparser;

use iron::prelude::*;

use iron::headers::{ ContentType };

pub fn check_content_type(request: &Request) -> Result<(), String> {
    let content_type = request.headers.get::<ContentType>().unwrap();
    if content_type != &ContentType::png() {
        return Err("content-type is not 'image/png'".to_string());
    }
    Ok(())
}
