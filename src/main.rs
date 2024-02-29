use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_files::Files;
use std::fs;

async fn index() -> impl Responder {
   let html = fs::read_to_string("app/index.html")
   .expect("Something went wrong reading the file");

   HttpResponse::Ok().body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(
            Files::new("/static", "./app/style.css")
        )
        .service(
            web::scope("/app")
                .route("/index.html", web::get().to(index)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}