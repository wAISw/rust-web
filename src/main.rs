#![allow(unreachable_code)]
#![allow(unused_variables)]

#[macro_use]
extern crate rocket;

mod state;
use sqlx::postgres::PgPoolOptions;
mod errors;
use errors::*;
mod routes;
use dotenvy::dotenv;
use routes::{authorize, index, refund};
use std::env;

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
    dotenv().ok();
    let database_url = env::var("DATABASE_URL");
    println!("{}", database_url.clone().unwrap());
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url.unwrap()[..])
        .await
        .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS actions_queue (
          id VARCHAR PRIMARY KEY NOT NULL,
          action_type VARCHAR NOT NULL,
          account VARCHAR NOT NULL,
          amount REAL NOT NULL,
          created_at timestamp with TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
        );"#,
    )
    .execute(&pool)
    .await
    .unwrap();

    let app_state = state::AppState::new();

    rocket::build()
        .mount("/", routes![index, authorize, refund])
        .register("/", catchers![internal_server_error])
        .manage(pool)
        .manage(app_state)
}
