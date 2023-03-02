use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use ZeroToProd::run;
use std::net::TcpListener;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    run(listener)?.await
}

// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     HttpServer::new(|| {
//         App::new()
//         .route("/", web::get().to(greet))
//         .route("/{name}", web::get().to(greet))
//
//     })
//     .bind("127.0.0.1:8000")?
//     .run()
//     .await
//}
// async fn health_check(req: HttpRequest) -> impl
// Responder {
//     HttpResponse::Ok()
// }

// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     HttpServer::new(|| {
//         App::new()
//             .route("/health_check",
//                    web::get().to(health_check))
//     })
//         .bind("127.0.0.1:8000")?
//         .run()
//         .await
// }

