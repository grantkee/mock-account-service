use super::{Account, AccountError};
use rocket::serde::{json::Json, uuid::Uuid};

#[post("/account/create", data = "<data>")]
async fn account_create(
    data: Json<Account>
) -> Result<Json<Account>, AccountError> {
    
    Ok(())
}

pub fn routes() -> Vec<Route> {
    routes![
        account_create,
    ]
}
