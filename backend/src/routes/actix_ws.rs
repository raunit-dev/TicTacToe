use actix_web::{HttpRequest, HttpServer, rt, web, App};
use actix_ws::Message;

async fn ws_handler(request: HttpRequest,body: web::Payload) -> Result<actix_web::HttpResponse, actix_web::error::Error> {
  let (response, mut session, mut stream) = actix_ws::handle(&request, body).map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
   
  rt::spawn(async move { //imp not here
    // rt::spawn -> this spawn a task(not a thread) that keeps checking if
    // there is an incoming message 
    while let Some(message) = stream.recv().await {
        // when awaiting, this task yeilds the threads 
        // thread can handle other tasks while waiting
        match message.unwrap() {
            Message::Ping(data) => {
                let _ = session.pong(&data).await;
            }
            Message::Text(message) => {
                let _ = session.text(message).await;
            }

            _=> {

            }
        }
    }
  });

  Ok(response)

}

// not here actix_web::main creates a multi-threaded runtime with 4 worker threads (by default, based on CPU cores)

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