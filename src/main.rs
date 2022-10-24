use dotenvy::dotenv;
use std::{env, sync::Arc};

use domain::service::ApplicationService;
use infra::http;

mod domain;
mod infra;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url.as_str())
        .await
        .unwrap();

    let item_repo = Arc::new(infra::repositories::SqlxItemRepository::new(pool.clone()));
    let broker = Arc::new(infra::broker::AMQPBroker::new().await);
    let app_service = Arc::new(ApplicationService::new(item_repo, broker));

    let options = http::HttpOptions {
        host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        port: env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .unwrap(),
    };
    http::setup(app_service, options).await.unwrap();
}
