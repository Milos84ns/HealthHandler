
use axum::{routing::get, Router};
use lazy_static::lazy_static;
use std::sync::{Arc};
use health_handler::health::{AppState, HealthService};
use std::borrow::Borrow;

lazy_static! {
    pub static ref HEALTH_SERVICE: Arc<HealthService> = Arc::new(HealthService::new(
        String::from("ExampleApp"),
        String::from("Simple UI template"),
        env!("CARGO_PKG_VERSION").parse().unwrap(),
        AppState::NotRunning,
        ));
}


#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // build our application with a route
    println!("Health service started http://localhost:{}/health",3000);
    let app = Router::new().route("/health", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> String {
    serde_json::to_string(&HEALTH_SERVICE.get_health()).unwrap().clone()
}