use tokio;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpServer) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to_get(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
