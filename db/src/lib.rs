use anyhow::Result;
use std::env;
pub mod models;
use sqlx::{
    postgres::{PgPool, PgPoolOptions},
};

#[derive(Clone)]
pub struct Store {
    pool: PgPool,
}

impl Store {
    pub async fn new() -> Result<Self> {
        let db_url = env::var("DATABASE_URL")?;
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;

        Ok(Self { pool })
    }
}


//we want humare database he prisma defined ho which is why 
//whenever you create a package most probably it will export a struct
//diesel and sqlx in both we have to create manual migration
//sqlx-migrate in order to have automatic migration
//sqlx is sync by default to make it async you need to import features which is runtime-tokio