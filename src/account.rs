use thiserror::Error;
use uuid::Uuid;
use sqlx::{query, AnyConnection};
use serde::{Serialize, Deserialize};
use rocket::response::{self, Responder, Response};

#[derive(Serialize, Deserialize)]
pub struct Account {
    id: Uuid,
    username: String,
}

#[derive(Error, Debug)]
pub enum AccountError {
    #[error("Account already exists.")]
    AccountExists,
    #[error("Database error: {0:}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Unknown error has occurred.")]
    UnknownError,
}


impl Account {
    pub async fn create(
        connection: &mut AnyConnection,
        username: &str,
    ) -> Result<Account, AccountError> {
        let id = Uuid::new_v4();
        let result = query("INSERT INTO account(username) VALUES(?)")
            .bind(username)
            .execute(connection)
            .await?;
        Ok(Account{id: id, username: username.to_owned()})
    }
}
