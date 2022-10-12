use domain::commands::{Command, CreateItem, CreateItemHandler};
use infra::repositories::SqlxItemRepository;
use sqlx::postgres::PgPoolOptions;

use crate::domain::queries::{GetItem, GetItemHandler, Query};

mod domain;
mod infra;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:example@localhost:5432/catalog")
        .await?;
    let create_item_handler =
        CreateItemHandler::new(Box::new(SqlxItemRepository::new(pool.clone())));
    let cmd = CreateItem::new(
        "Name".to_string(),
        "Description".to_string(),
        bigdecimal::FromPrimitive::from_f32(1.0).unwrap(),
        "image_url".to_string(),
    );
    let id = create_item_handler.handle(cmd).await.unwrap();
    println!("id: {}", id);
    // Get Item
    let get_item_handler = GetItemHandler::new(Box::new(SqlxItemRepository::new(pool.clone())));
    let query = GetItem::new(id);
    let item = get_item_handler.handle(query).await.unwrap();
    println!("item: {:?}", item);
    Ok(())
}
