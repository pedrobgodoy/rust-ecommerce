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
    use std::{future, sync::Arc};

    use bigdecimal::{BigDecimal, FromPrimitive};
    use mockall::predicate::*;

    use crate::domain::repositories::MockItemRepository;

    use super::*;

    #[tokio::test]
    async fn it_should_get_item() {
        let mut item_repo_mock = MockItemRepository::new();

        let item = Item::new(
            "id".to_string(),
            "name".to_string(),
            "description".to_string(),
            BigDecimal::from_f32(1.0).unwrap().clone(),
            "image_url".to_string(),
        );
        let item_clone = item.clone();

        item_repo_mock
            .expect_find_by_id()
            .with(eq(item.id.clone()))
            .times(1)
            .returning(move |_| Box::pin(future::ready(Ok(item_clone.clone()))));

        let handler = GetItemHandler::new(Arc::new(item_repo_mock));
        let query = GetItem::new(item.id.clone());
        let result = handler.handle(query).await.unwrap();

        assert_eq!(result.id, item.id.clone());
        assert_eq!(result.name, item.name.clone());
        assert_eq!(result.description, item.description.clone());
        assert_eq!(result.price, item.price.clone());
        assert_eq!(result.image_url, item.image_url.clone());
    }
}
