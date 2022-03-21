use actix_web::web::Form;
use actix_web::HttpResponse;

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
pub async fn subscribe(form: Form<FormData>) -> HttpResponse {
    println!("New subscriber: email: {}, name {}", form.email, form.name);
    HttpResponse::Ok().finish()
}
