use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::fs;

#[get("/")]
async fn hello() -> impl Responder {
    // HttpResponse::Ok().body("Hello world!")

    let content = fs::read_to_string("templates/index.html")
        .expect("Something went wrong reading the file");

    // Serve the content as HTML
    HttpResponse::Ok()
        .content_type("text/html")
        .body(content)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}