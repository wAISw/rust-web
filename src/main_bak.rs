#[macro_use]
extern crate rocket;

use rocket::{
    serde::json::Json,
    serde::Deserialize,
    serde::{json::serde_json::Error, Serialize}, response::status,
};
use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response, Result};
use rocket::request::Request;

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

#[derive(Debug)]
pub struct UnprocessableEntityError {
    message: String,
}

impl UnprocessableEntityError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl<'r> Responder<'r, 'static> for UnprocessableEntityError {
    fn respond_to(self, _: &Request<'_>) -> Result<'r> {
        let response_body = serde_json::json!({
            "message": self.message
        });
        Response::build()
            .status(Status::InternalServerError)
            .header(ContentType::JSON)
            .sized_body(response_body.to_string().len(), std::io::Cursor::new(response_body.to_string()))
            .ok()
    }
}

impl From<diesel::result::Error> for UnprocessableEntityError {
    fn from(error: diesel::result::Error) -> Self {
        if let diesel::result::Error::DatabaseError(_, ref err) = error {
            if err.code == diesel::result::DatabaseErrorKind::UniqueViolation {
                return Self::new(String::from("Unprocessable Entity Error"));
            }
        }
        Self::new(String::from("Internal Server Error"))
    }
}

#[post("/authorize", data = "<data>")]
pub fn authorize(data: Json<AuthData>) -> status::Custom<String> {
    match some_validation(&data) {
        Ok(_) => status::Custom(Status::Ok, String::from("Everything is OK!")),
        Err(e) => status::Custom(Status::BadRequest, String::from("Everything is not OK!")),
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RefundData {
    pub account: String,
    pub amount: u32,
}

#[post("/refund", format = "json", data = "<data>")]
pub fn refund(data: Json<RefundData>) -> String {
    print!("{:#?}", data);
    format!("ok")
}

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

#[catch(422)]
fn unprocessable_entity() -> UnprocessableEntityError {
    UnprocessableEntityError::new(String::from("Unprocessable Entity Error"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![internal_error, not_found, unprocessable_entity])
        .mount("/", routes![index])
        .mount("/", routes![authorize])
        .mount("/", routes![refund])
}
