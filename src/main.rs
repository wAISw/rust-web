#[macro_use]
extern crate rocket;

use rocket::{
    http::Status,
    serde::json::Json,
    serde::Deserialize,
    serde::{json::serde_json::Error, Serialize},
    Request,
};

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
pub fn authorize(data: Json<AuthData>) -> Result<Json<AuthData>, Status> {
    print!("{:#?}", data);
    Ok(data)
    // match data.is_ok() {
    //     Ok(data) => Ok(Json(data)),
    //     Err(_) => Err(Status::InternalServerError),
    // }
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
fn validation_error(status: Status, req: &Request) -> String {
    // format!("{} ({})", status, req.uri())
    return InternalServerError.into();
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![internal_error, not_found, validation_error])
        .mount("/", routes![index])
        .mount("/", routes![authorize])
        .mount("/", routes![refund])
}
