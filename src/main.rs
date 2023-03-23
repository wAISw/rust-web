#[macro_use]
extern crate rocket;

use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};

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

#[post("/authorize", format = "json", data = "<data>")]
pub fn authorize(data: Json<AuthData>) -> String {
    print!("{:#?}",data);
    format!("ok")
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RefundData {
    pub account: String,
    pub amount: u32,
}

#[post("/refund", format = "json", data = "<data>")]
pub fn refund(data: Json<RefundData>) -> String {
    print!("{:#?}",data);
    format!("ok")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![authorize])
        .mount("/", routes![refund])
}
