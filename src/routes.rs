use crate::errors::*;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[get("/")]
pub(crate) async fn index(pool: &rocket::State<PgPool>) -> &'static str {
    let result = sqlx::query!("SELECT * FROM actions_queue")
        .fetch_all(pool.inner())
        .await
        .unwrap();
    let return_value = "Hello, world!";
    return_value
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthData {
    pub account: String,
    pub amount: f32,
}

/*
curl -d '{"account":"test", "amount":42}' -H "Content-Type: application/json" -X POST http://127.0.0.1:8000/authorize
*/
#[post("/authorize", format = "json", data = "<data>")]
pub(crate) async fn authorize(
    pool: &rocket::State<PgPool>,
    data: Json<AuthData>,
) -> RestApiResult<Json<StatusResponse>, RestApiError> {
    let id = uuid::Uuid::new_v4().to_string();
    let result = sqlx::query!(
        r#"
        INSERT INTO actions_queue(id, action_type, account, amount)
        VALUES ($1, 'authorize', $2, $3)
        RETURNING *;
        "#,
        id,
        data.account.clone(),
        data.amount.clone(),
    )
    .fetch_all(pool.inner())
    .await
    .unwrap();

    Ok(Json(StatusResponse {
        status: String::from("authorized"),
    }))
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RefundData {
    pub account: String,
    pub amount: f32,
}

#[post("/refund", format = "json", data = "<data>")]
pub(crate) async fn refund(
    pool: &rocket::State<PgPool>,
    data: Json<RefundData>,
) -> RestApiResult<Json<StatusResponse>, RestApiError> {
    let id = uuid::Uuid::new_v4().to_string();
    let result = sqlx::query!(
        r#"
        INSERT INTO actions_queue(id, action_type, account, amount)
        VALUES ($1, 'refund', $2, $3)
        RETURNING *;
        "#,
        id,
        data.account.clone(),
        data.amount.clone(),
    )
    .fetch_all(pool.inner())
    .await
    .unwrap();

    Ok(Json(StatusResponse {
        status: String::from("authorized"),
    }))
}
