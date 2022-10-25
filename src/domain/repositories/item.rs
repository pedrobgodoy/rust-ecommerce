use crate::domain::entities::Item;
use async_trait::async_trait;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[derive(Debug)]
pub enum ItemRepositoryError {
    NotFound,
    Generic(String),
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait ItemRepository {
    async fn save(&self, item: Item) -> Result<(), ItemRepositoryError>;
    async fn find_by_id(&self, id: String) -> Result<Item, ItemRepositoryError>;
}
