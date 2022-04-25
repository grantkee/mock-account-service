use structopt::StructOpt;
use std::net::Ipv4Addr;
use sqlx::AnyPool;
use anyhow::Result;
use rocket::{tokio, Build, Rocket};

#[derive(StructOpt, Clone, Debug)]
#[structopt(name = "account service", about = "An example account service for a microservices backend.")]
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

    pub async fn rocket(&self) -> Result<Rocket<Build>> {
        todo!()
    }
}

#[tokio::main]
async fn main() {
    let options = Options::from_args();
    println!("Hello, world!");
}
