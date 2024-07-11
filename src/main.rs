use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;

use api::home;

mod api;
mod models;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(home);
    };

    Ok(config.into())
}
