use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

async fn goodbye(req: HttpRequest) -> impl Responder {
    let id = req.match_info().get("id").unwrap_or("World");
    format!("Goodbye {}!", id)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/hello/{name}", web::get().to(greet))
            .route("/goodbye/{id}", web::get().to(goodbye))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
