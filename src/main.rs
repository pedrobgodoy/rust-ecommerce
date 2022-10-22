use dotenvy::dotenv;
use std::{env, sync::Arc};

use domain::service::ApplicationService;
use infra::http;

mod domain;
mod infra;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var_os("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url.to_str().unwrap())
        .await
        .unwrap();

    let item_repo = Arc::new(infra::repositories::SqlxItemRepository::new(pool.clone()));
    let app_service = Arc::new(ApplicationService::new(item_repo));

    http::setup(app_service).await.unwrap();
}
