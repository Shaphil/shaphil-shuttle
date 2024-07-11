use actix_web::{get, HttpResponse, Responder};
use crate::models::User;

#[get("/")]
async fn home() -> impl Responder {
    let user = User::new(
        "Md. Abdullah Al".to_string(),
        "Mahmud".to_string(),
        String::from("shaphil"),
        String::from("admin@email.com"),
    );

    return HttpResponse::Ok()
        .content_type("application/json")
        .json(user);
}
