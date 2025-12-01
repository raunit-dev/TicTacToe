use actix_web::{HttpServer,App,web,rt,HttpResponse,HttpRequest};
use actix_web::error::Error;
use actix_ws::{Message};

async fn ws_handler(req: HttpRequest, body: web::Payload) -> Result<HttpResponse,Error> {
   let (response,mut session,mut stream) = actix_ws::handle(&req, body).unwrap();
   rt::spawn(async move {
      while let Some(message) = stream.recv().await {
        match message.unwrap() {
            Message::Ping(data) => {
                let _ = session.pong(&data).await;
            }
            Message::Text(message) => {
                let _ = session.text(message).await;
            }
            _ => {

            }
        }
      }
   });
   Ok(response)
}

#[actix_web::main]
async fn main() {
    let _ = HttpServer::new(|| {
        App::new()
            .route("/ws", web::get().to(ws_handler))
    })
    .bind("0.0.0.0:3000")
    .unwrap()
    .run()
    .await;
}