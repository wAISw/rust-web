#![allow(unreachable_code)]
#![allow(unused_variables)]

#[macro_use]
extern crate rocket;

mod state;
use state::*;
mod errors;
use errors::*;
mod routes;
use routes::*;

use routes::{authorize, index, refund};

#[derive(Debug)]
pub struct ErrorResponder {
    message: String,
}

impl ErrorResponder {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

#[catch(422)]
fn internal_server_error() -> RestApiError {
    RestApiError::RestApiServiceError {
        source: "Internal Sereve Error".to_string(),
    }
}

#[launch]
async fn rocket() -> _ {
    let app_state = state::AppState::new();

    rocket::build()
        .mount("/", routes![index, authorize, refund])
        .register("/", catchers![internal_server_error])
        .manage(app_state)
}
