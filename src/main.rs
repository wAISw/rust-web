#![allow(unreachable_code)]
#![allow(unused_variables)]

#[macro_use]
extern crate rocket;

mod state;
use sqlx::query;
use sqlx::{postgres::PgPoolOptions, PgPool};
use state::*;
mod errors;
use errors::*;
mod routes;
use dotenvy::dotenv;
use rocket::http::Status;
use rocket::State;
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
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url.unwrap()[..])
        .await
        .unwrap();

    sqlx::query(
        r#"
    CREATE TABLE IF NOT EXISTS actions_queue_sqlx (
      id VARCHAR PRIMARY KEY NOT NULL,
      action_type VARCHAR NOT NULL,
      data VARCHAR NOT NULL,
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
