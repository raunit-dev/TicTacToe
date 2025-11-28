use anyhow::Resultl
use actix_web::
use db::Db;

async fn main() {
    let db = unsafe { Db::new().await.unwrap();

   db.create_user();

   println!("{}",s);
}