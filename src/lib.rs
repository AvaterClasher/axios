//! src/lib.rs

use actix_web::dev::Server;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[get("/health_check")]
async fn health_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check))
        .bind("127.0.0.1:8000")?
        .run();
    Ok(server)
}
