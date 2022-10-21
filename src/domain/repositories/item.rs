use crate::domain::entities::Item;
use async_trait::async_trait;

#[derive(Debug)]
pub enum ItemRepositoryError {
    Generic(String),
}

#[async_trait]
pub trait ItemRepository {
    async fn save(&self, item: Item) -> Result<(), ItemRepositoryError>;
    async fn find_by_id(&self, id: String) -> Result<Item, ItemRepositoryError>;
}
