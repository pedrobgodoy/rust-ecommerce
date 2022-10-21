use std::sync::Arc;

use async_trait::async_trait;

use super::Query;
use crate::domain::{
    entities::Item,
    repositories::{ItemRepository, ItemRepositoryError},
};

pub struct GetItem {
    pub id: String,
}

impl GetItem {
    pub fn new(id: String) -> GetItem {
        GetItem { id }
    }
}

pub struct GetItemHandler {
    item_repo: Arc<dyn ItemRepository + Send + Sync>,
}

impl GetItemHandler {
    pub fn new(item_repo: Arc<dyn ItemRepository + Send + Sync>) -> GetItemHandler {
        GetItemHandler { item_repo }
    }
}

#[async_trait]
impl Query<GetItem, Result<Item, ItemRepositoryError>> for GetItemHandler {
    async fn handle(&self, query: GetItem) -> Result<Item, ItemRepositoryError> {
        self.item_repo.find_by_id(query.id).await
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    use bigdecimal::{BigDecimal, FromPrimitive};

    use crate::{
        domain::{entities::Item, queries::Query, repositories::ItemRepository},
        infra::repositories::InMemoryItemRepository,
    };

    use super::{GetItem, GetItemHandler};

    #[tokio::test]
    async fn it_should_get_item() {
        let item_store = Arc::new(Mutex::new(HashMap::<String, Item>::new()));
        let item_repo = Box::new(InMemoryItemRepository::new(Arc::clone(&item_store)));
        let handler = GetItemHandler::new(Arc::new(InMemoryItemRepository::new(Arc::clone(
            &item_store,
        ))));
        let id = "id".to_string();
        let name = "name".to_string();
        let description = "description".to_string();
        let price = BigDecimal::from_f32(1.0).unwrap().clone();
        let image_url = "image_url".to_string();
        let item = Item::new(
            id.clone(),
            name.clone(),
            description.clone(),
            price.clone(),
            image_url.clone(),
        );
        item_repo.save(item).await.unwrap();

        let query = GetItem::new(id.clone());
        let result = handler.handle(query).await.unwrap();

        assert_eq!(result.id, id);
        assert_eq!(result.name, name);
        assert_eq!(result.description, description);
        assert_eq!(result.price, price);
        assert_eq!(result.image_url, image_url);
    }
}
