#![allow(dead_code)]
pub mod health;
pub mod health_dependency;

#[cfg(test)]
mod tests {
    use crate::health::{AppState,Health};
    use crate::health_dependency::HealthDependency;
    use ::axum::routing::get;
    use ::axum::Router;
    use ::axum_test::TestServer;

    #[tokio::test]
    async fn it_should_get_health() {
           let mut health = Health::new(
                    env!("CARGO_PKG_NAME").to_string(),
                    env!("CARGO_PKG_DESCRIPTION").to_string(),
                    env!("CARGO_PKG_VERSION").to_string(),
                    AppState::Started
           );

          health.set_available(true);
        health.update_health(
            vec![
                HealthDependency::example(true),
                HealthDependency::example(false),
            ]
        );

        health.set_state(AppState::Running);

        let app = Router::new().route("/health",get(health_handle(health)));

        let server = TestServer::new(app).unwrap();

        let response = server.get("/health").await;

        dbg!(&response);
        let health_response: Health = serde_json::from_str(&response.text()).unwrap();

        // should fail since there is one bad dependency
        assert_eq!(health_response.component,env!("CARGO_PKG_NAME").to_string());
        assert_eq!(health_response.is_available, false);
    }
    fn health_handle(health: Health) -> String {
        return serde_json::to_string(&health).unwrap();
    }
}