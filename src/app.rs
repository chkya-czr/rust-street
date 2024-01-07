use crate::routes;
use axum::Router;

pub async fn create_app() -> Router {
    Router::new().merge(routes::order::create_route())
}
