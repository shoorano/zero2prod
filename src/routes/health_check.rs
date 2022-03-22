use actix_web::HttpResponse;

/// End point to check server status.
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
