use crate::{errors::*, state::AppState};
use rust_web::*;

use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres};

#[get("/")]
pub(crate) fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthData {
    pub account: String,
    pub amount: u32,
}

#[post("/authorize", format = "json", data = "<raw_data>")]
pub(crate) async fn authorize(
    // state: State<'_, PgPool>,
    raw_data: Json<AuthData>,
) -> RestApiResult<Json<StatusResponse>, RestApiError> {
    // print!("{:#?}", state);
    // use self::schema::actions_queue;
    // let connection = &mut establish_connection();

    // let id = uuid::Uuid::new_v4().to_string();

    // let data_s = serde_json::to_string(&raw_data.into_inner()).unwrap();
    // let new_action: NewAction = NewAction {
    //     action_type: &("authorize".to_string())[..],
    //     data: &data_s[..],
    // };

    // let created_action: Action = diesel::insert_into(actions_queue::table)
    //     .values(&new_action)
    //     .get_result::<Action>(connection)
    //     .expect("Error saving new role");

    // print!("{:#?}", created_action);
    Ok(Json(StatusResponse {
        status: String::from("authorized"),
    }))
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RefundData {
    pub account: String,
    pub amount: u32,
}

#[post("/refund", format = "json", data = "<data>")]
pub(crate) fn refund(data: Json<RefundData>) -> RestApiResult<Json<StatusResponse>, RestApiError> {
    let json_data: RefundData = RefundData {
        account: data.account.clone(),
        amount: data.amount.clone(),
    };
    let json = serde_json::to_string(&json_data).unwrap();
    println!("{json}");
    print!("{} - {}", data.account, data.amount);
    Ok(Json(StatusResponse {
        status: String::from("done"),
    }))
}
