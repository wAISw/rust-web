#[macro_use]
extern crate rocket;

use rocket::{
    http::{ContentType, Status},
    request::Request,
    response::{Responder, Response},
    serde::json::Json,
    serde::Deserialize,
    serde::Serialize,
};
use serde_json;

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

#[catch(422)]
fn internal_server_error() -> RocketErrorResponder {
    RocketErrorResponder::new(String::from("Internal Server Error"))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthData {
    pub account: String,
    pub amount: u32,
}

#[post("/authorize", data = "<data>")]
pub fn authorize(data: Json<AuthData>) -> Json<AuthData> {
    print!("{:#?}", data);
    data
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RefundData {
    pub account: String,
    pub amount: u32,
}

#[post("/refund", format = "json", data = "<data>")]
pub fn refund(data: Json<RefundData>) -> Json<RefundData> {
    print!("{:#?}", data);
    data
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![internal_server_error])
        .mount("/", routes![index])
        .mount("/", routes![error_route])
        .mount("/", routes![authorize])
        .mount("/", routes![refund])
}
