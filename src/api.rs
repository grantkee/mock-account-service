use crate::account;
use crate::account::{Account, AccountError};
use rocket::serde::{json::Json, uuid::Uuid};
use rocket::*;

#[post("/account/create", data = "<data>")]
async fn account_create(data: Json<Account>) -> Result<Json<Account>, AccountError> {
    match account::create(&data).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(e),
    }
}

pub fn routes() -> Vec<Route> {
    routes![account_create,]
}
