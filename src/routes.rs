use crate::errors::*;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

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

#[post("/authorize", format = "json", data = "<data>")]
pub(crate) async fn authorize(
    data: Json<AuthData>,
) -> RestApiResult<Json<StatusResponse>, RestApiError> {
    print!("{:#?}", data);
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
