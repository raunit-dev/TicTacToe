use anyhow::{Ok, Result};
use actix_web::{web, App, HttpServer};

pub mod routes;

use routes::user::{create_user, sign_in, me};

//Opting for explicit routing (no macros)
#[actix_web::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let store = db::Store::new().await?;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(store.clone()))
            .service(
                web::scope("/api/v1")
                    .service(web::resource("/signup").route(web::post().to(create_user)))
                    .service(web::resource("/signin").route(web::post().to(sign_in)))
                    .service(web::resource("/me").route(web::get().to(me)))
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}