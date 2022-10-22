use std::sync::Arc;

use domain::service::ApplicationService;
use infra::http;

mod domain;
mod infra;

#[tokio::main]
async fn main() {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect("postgres://postgres:example@localhost:5432/catalog")
        .await
        .unwrap();

    let item_repo = Arc::new(infra::repositories::SqlxItemRepository::new(pool.clone()));
    let app_service = Arc::new(ApplicationService::new(item_repo));

    http::setup(app_service).await.unwrap();
}
