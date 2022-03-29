use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgConnection;
use std::net::TcpListener;

/// returns a Result, if Ok then returns a HttpServer, otherwise returns a
/// std::io::Error
pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    // Required as PgConnection is not cloneable and a HttpServer creates
    // a copy of the closure for each of the host machines cores
    let connection = web::Data::new(connection);
    // Capture `connection` from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
