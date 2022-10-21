use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use sqlx::{Postgres, Row};

use crate::domain::{
    entities::Item,
    repositories::{ItemRepository, ItemRepositoryError},
};

pub struct InMemoryItemRepository {
    items_store: Arc<Mutex<HashMap<String, Item>>>,
}

impl InMemoryItemRepository {
    pub fn new(items_store: Arc<Mutex<HashMap<String, Item>>>) -> Self {
        InMemoryItemRepository { items_store }
    }
}

#[async_trait]
impl ItemRepository for InMemoryItemRepository {
    async fn save(&self, item: Item) -> Result<(), ItemRepositoryError> {
        let mut items = self.items_store.lock().unwrap();
        items.insert(item.id.clone(), item);
        Ok(())
    }
    async fn find_by_id(&self, id: String) -> Result<Item, ItemRepositoryError> {
        let items = self.items_store.lock().unwrap();
        let item = items.get(&id).unwrap();
        Ok(item.clone())
    }
}

pub struct SqlxItemRepository {
    pool: sqlx::Pool<Postgres>,
}

impl SqlxItemRepository {
    pub fn new(pool: sqlx::Pool<Postgres>) -> Self {
        SqlxItemRepository { pool }
    }
}

#[async_trait]
impl ItemRepository for SqlxItemRepository {
    async fn save(&self, item: Item) -> Result<(), ItemRepositoryError> {
        let result = sqlx::query("INSERT INTO items (id, name, description, price, image_url, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(item.id)
            .bind(item.name)
            .bind(item.description)
            .bind(item.price)
            .bind(item.image_url)
            .bind(item.created_at)
            .bind(item.updated_at)
            .execute(&self.pool)
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(ItemRepositoryError::Generic(e.to_string())),
        }
    }
    async fn find_by_id(&self, id: String) -> Result<Item, ItemRepositoryError> {
        let result = sqlx::query("SELECT * FROM items WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await;
        match result {
            Ok(row) => Ok(Item {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                price: row.get("price"),
                image_url: row.get("image_url"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }),
            Err(e) => Err(ItemRepositoryError::Generic(e.to_string())),
        }
    }
}
