use log::*;
use rocket::http::Status;
use rocket::response::{self, Responder, Response};
use serde::{Deserialize, Serialize};
use sqlx::{query, AnyConnection};
use thiserror::Error;
use uuid::Uuid;

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

impl<'r> Responder<'r, 'static> for AccountError {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> response::Result<'static> {
        error!("Error: {}", self);
        use AccountError::*;
        match self {
            AccountExists => Response::build().status(Status::Forbidden).ok(),
            DatabaseError(_) => Response::build().status(Status::BadRequest).ok(),
            _ => Response::build().status(Status::BadRequest).ok(),
        }
    }
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
        Ok(Account {
            id: id,
            username: username.to_owned(),
        })
    }
}

// pub async fn create()
