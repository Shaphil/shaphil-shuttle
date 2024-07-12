use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;

use api::{home, get_users};

mod api;
mod models;
mod db;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg
            .service(home)
            .service(get_users);
    };

    Ok(config.into())
}
