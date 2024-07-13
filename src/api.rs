use actix_web::{get, HttpResponse, Responder};
use crate::models::User;
use crate::db;

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

#[get("/api/v1/users")]
pub async fn get_users() -> impl Responder {
    match db::get_users().await {
        Some(users) => {
            return HttpResponse::Ok()
                .content_type("application/json")
                .json(users);
        }
        None => {
            HttpResponse::NoContent()
                .body("There are no users in the system")
        }
    }
}
