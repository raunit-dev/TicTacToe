use actix_web::{App, HttpServer, web};
use anyhow::{Ok, Result};
pub mod middleware;
pub mod routes;
use routes::user::{create_user, get_user, sign_in};

//Opting for explicit routing (no macros)
#[actix_web::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let store = db::Store::new().await?;
    HttpServer::new(move || {
        App::new().app_data(web::Data::new(store.clone())).service(
            web::scope("/api/v1")
                .service(web::resource("/signup").route(web::post().to(create_user)))
                .service(web::resource("/signin").route(web::post().to(sign_in)))
                .service(web::resource("/me").route(web::get().to(get_user))),
        )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}
