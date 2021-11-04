use std::net::SocketAddr;
use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Method, Request, Response, Server, StatusCode};

extern crate serde_json;
use serde_json::json;

extern crate image;
extern crate driver_99bugs_display;

use driver_99bugs_display::Display;
use image::ImageFormat;

// This is our service handler. It receives a Request, routes on its
// path, and returns a Future of a Response.
async fn display_service(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
  match (req.method(), req.uri().path()) {
      (&Method::GET, "/") => {
        let result = json!({
          "message": "Welcome to 99 Bugs LED Display API",
          "help": "Just post image/png as raw to / to display it. Make sure dimensions are within limits."
        });

        Ok(Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(result.to_string()))
            .unwrap()
          )
      }

      // Need to await full body before we can handle request !
      (&Method::POST, "/") => {
          let whole_body = hyper::body::to_bytes(req.into_body()).await?;

          match image::load_from_memory_with_format(&whole_body, ImageFormat::Png) {
            Ok(image) => {
                let image = image.to_rgb8();
                let (width, height) = image.dimensions();
                if height > 64 || width > 96 {
                  return Ok(Response::builder()
                    .status(StatusCode::PAYLOAD_TOO_LARGE)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(json!({"message": "Dimensions cannot fit display. Please pick smaller image." }).to_string()))
                    .unwrap()
                  )
                }
    
                let mut display = Display::new("/dev/spidev0.0");
                match display.write_frame(&image.as_raw()) {
                  Ok(_) => {},
                  Err(err) => { 
                    return Ok(Response::builder()
                      .status(StatusCode::INTERNAL_SERVER_ERROR)
                      .header(header::CONTENT_TYPE, "application/json")
                      .body(Body::from(json!({"message": "Failed to send image to display", "error": err.to_string() }).to_string()))
                      .unwrap()
                    )
                   }
                };
            },
            Err(err) => {
              return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"message": "Failed to decode image", "error": err.to_string() }).to_string()))
                .unwrap()
              )
            }
          };

          Ok(Response::builder()
              .status(StatusCode::OK)
              .header(header::CONTENT_TYPE, "application/json")
              .body(Body::from(json!({"message": "Successfully send image to display." }).to_string()))
              .unwrap()
            )
      }

      // Return the 404 Not Found for other routes.
      _ => {
          let mut not_found = Response::default();
          *not_found.status_mut() = StatusCode::NOT_FOUND;
          Ok(not_found)
      }
  }
}

async fn shutdown_signal() {
  // Wait for the CTRL+C signal
  tokio::signal::ctrl_c()
      .await
      .expect("failed to install CTRL+C signal handler");

  println!("Closing down server ...")
}

#[tokio::main]
async fn main() {
  println!("99Bugs LED display API listening at port 3000");
  let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

  // A `Service` is needed for every connection.
  let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(display_service)) });

  let server = Server::bind(&addr).serve(service);

  // Add a graceful shutdown signal...
  let graceful = server.with_graceful_shutdown(shutdown_signal());

  // Run this server for... forever!
  if let Err(e) = graceful.await {
    eprintln!("server error: {}", e);
  }
}