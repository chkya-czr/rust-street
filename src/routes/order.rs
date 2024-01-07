use axum::{routing::post, Router};

pub fn create_route() -> Router {
    Router::new().route("/order", post(create_order))
}

async fn create_order() {}
