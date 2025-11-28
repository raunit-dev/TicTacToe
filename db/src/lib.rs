use anyhow::Result;
use serde::{Serialize,Deserialize};
use std::env;

use sqlx::{Postgres,postgres::{PgPool, PgPoolOptions}};
struct Store {
    pool: PgPool
}

impl Store {
    pub async fn new() -> Result<Self> {
        let db_url = env::var("DATABASE_URL")?;
        let pool = PgPoolOptions::new()
                 .max_connections(5)
                 .connect(&db_url).await?;

                Ok(Self {
                    pool
                })
    }
}

