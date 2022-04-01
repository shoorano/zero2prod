use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

/// Struct that contains required information a user needs to provide to
/// subscribe.
#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    name: String,
    email: String,
}

/// Endpoint that receives urlencoded request data is then to be converted to
/// FormData which utilises serde::Deserialize macro to return a 400 status
/// HttpResponse if data cannot be converted to FormData
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
