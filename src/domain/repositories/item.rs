use crate::domain::entities::Item;
use async_trait::async_trait;

#[async_trait]
pub trait ItemRepository {
    async fn save(&self, item: Item) -> Result<(), ()>;
    async fn find_by_id(&self, id: String) -> Result<Item, ()>;
}
