use anyhow::Result;
use rocket::{tokio, Build, Rocket};
use sqlx::AnyPool;
use std::net::Ipv4Addr;
use structopt::StructOpt;

mod account;
mod api;

#[derive(StructOpt, Clone, Debug)]
enum Command {
    Run(Options),
    Migrate { database: String },
}

#[derive(StructOpt, Clone, Debug)]
#[structopt(
    name = "account service",
    about = "An example account service for a microservices backend."
)]

struct Options {
    /// Database to use.
    #[structopt(long, short, env = "DATABASE", default_value = "sqlite://:memory:")]
    database: String,
    /// Address to use.
    #[structopt(long, short, env = "ADDRESS", default_value = "0.0.0.0")]
    address: Ipv4Addr,
    /// Port to use.
    #[structopt(long, short, env = "PORT", default_value = "3000")]
    port: u16,
}

impl Options {
    pub async fn database(&self) -> Result<AnyPool> {
        // connect to database
        let pool = AnyPool::connect(&self.database).await?;
        // run migrations
        sqlx::migrate!().run(&pool).await?;
        Ok(pool)
    }

    pub async fn build_rocket(&self) -> Result<Rocket<Build>> {
        // build REST api using routes from api mod
        let rocket = rocket::build()
            .mount("api/v1", api::routes())
            .manage(self.clone());
        Ok(rocket)
    }

    pub async fn run(&self) -> Result<()> {
        // launch rocket
        let rocket = self.build_rocket().await?;
        rocket.launch().await?;
        Ok(())
    }
}

#[rocket::main]
async fn main() -> Result<()> {
    env_logger::init();
    let command = Command::from_args();

    match command {
        Command::Run(options) => {
            options.run().await?;
            Ok(())
        }
        Command::Migrate { database } => {
            let pool = AnyPool::connect(&database).await?;
            sqlx::migrate!().run(&pool).await?;
            Ok(())
        }
    }
}
