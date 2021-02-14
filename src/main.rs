use actix_web::middleware::{normalize::TrailingSlash, NormalizePath};
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

mod chat;
mod translate;

async fn greet(_: HttpRequest) -> impl Responder {
  String::from("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .wrap(NormalizePath::new(TrailingSlash::Trim))
      .route("/", web::get().to(greet))
      .route("/chat", web::get().to(chat::index))
      .route("/translate", web::get().to(translate::index))
  })
  .bind(("0.0.0.0", 4000))?
  .run()
  .await
}
