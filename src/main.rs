#[macro_use]
extern crate rocket;

use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{Responder, Response};

#[derive(Debug)]
pub struct RocketErrorResponder {
    message: String,
}

impl RocketErrorResponder {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl<'r> Responder<'r, 'static> for RocketErrorResponder {
    fn respond_to(self, _: &Request<'_>) -> rocket::response::Result<'static> {
        let response_body = serde_json::json!({
            "message": self.message
        });
        Response::build()
            .status(Status::InternalServerError)
            .header(ContentType::JSON)
            .sized_body(
                response_body.to_string().len(),
                std::io::Cursor::new(response_body.to_string()),
            )
            .ok()
    }
}

#[get("/error")]
fn error_route() -> Result<(), RocketErrorResponder> {
    // simulate an error
    Err(RocketErrorResponder::new(String::from(
        "An error occurred.",
    )))
}

#[catch(500)]
fn internal_server_error() -> RocketErrorResponder {
    RocketErrorResponder::new(String::from("Internal Server Error"))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![internal_server_error])
        .mount("/", routes![index])
        .mount("/", routes![error_route])

}
