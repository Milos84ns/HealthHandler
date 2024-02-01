use actix_web::{web, App, HttpServer, Responder};
use lazy_static::lazy_static;
use std::sync::{Arc};

use health_handler::health::{AppState, HealthService};

lazy_static! {
    pub static ref HEALTH_SERVICE: Arc<HealthService> = Arc::new(HealthService::new(
        String::from("ExampleApp"),
        String::from("Simple UI template"),
        env!("CARGO_PKG_VERSION").parse().unwrap(),
        AppState::NotRunning,
        ));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Health service started http://localhost:{}/health",8080);

    HttpServer::new(|| App::new().route("/health", web::get().to(health_handler)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn health_handler() -> impl Responder {
    return web::Json(HEALTH_SERVICE.get_health());
}