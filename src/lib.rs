use actix_web::dev::Server;
use actix_web::web::Form;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

/// End point to check server status.
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

/// Struct that contains required information a user needs to provide to
/// subscribe.
#[derive(serde::Deserialize, Debug)]
struct FormData {
    name: String,
    email: String,
}

/// Endpoint that receives urlencoded request data is then to be converted to
/// FormData which utilises serde::Deserialize macro to return a 400 status
/// HttpResponse if data cannot be converted to FormData
async fn subscribe(form: Form<FormData>) -> HttpResponse {
    println!("New subscriber: email: {}, name {}", form.email, form.name);
    HttpResponse::Ok().finish()
}

/// returns a Result, if Ok then returns a HttpServer, otherwise returns a
/// std::io::Error
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
