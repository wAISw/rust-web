use crate::schema::actions_queue;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Action {
    pub id: String,
    pub action_type: String,
    pub data: String,
    pub data1: String,
    pub created_at: DateTime<Utc>,
    // pub updated_at: String,
}

#[derive(Insertable)]
#[diesel(table_name = actions_queue)]
pub struct NewAction<'a> {
    pub action_type: &'a str,
    pub data: &'a str,
}
