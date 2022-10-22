use std::sync::Arc;

use async_trait::async_trait;
use bigdecimal::BigDecimal;
use uuid::Uuid;

use super::Command;
use crate::domain::{
    entities::Item,
    repositories::{ItemRepository, ItemRepositoryError},
};

#[derive(Debug, PartialEq)]
pub struct CreateItem {
    pub name: String,
    pub description: String,
    pub price: BigDecimal,
    pub image_url: String,
}

impl CreateItem {
    pub fn new(
        name: String,
        description: String,
        price: BigDecimal,
        image_url: String,
    ) -> CreateItem {
        CreateItem {
            name,
            description,
            price,
            image_url,
        }
    }
}

pub struct CreateItemHandler {
    item_repo: Arc<dyn ItemRepository + Send + Sync>,
}

impl CreateItemHandler {
    pub fn new(item_repo: Arc<dyn ItemRepository + Send + Sync>) -> CreateItemHandler {
        CreateItemHandler { item_repo }
    }
}

#[async_trait]
impl Command<CreateItem, Result<String, ItemRepositoryError>> for CreateItemHandler {
    async fn handle(&self, cmd: CreateItem) -> Result<String, ItemRepositoryError> {
        let id = Uuid::new_v4();
        let item = Item::new(
            id.to_string(),
            cmd.name,
            cmd.description,
            cmd.price,
            cmd.image_url,
        );
        self.item_repo.save(item).await?;
        Ok(id.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::future;

    use bigdecimal::{BigDecimal, FromPrimitive};

    use crate::domain::repositories::MockItemRepository;

    use super::*;

    #[tokio::test]
    async fn it_should_handle_command() {
        let cmd = CreateItem::new(
            "name".to_string(),
            "description".to_string(),
            BigDecimal::from_f32(1.0).unwrap(),
            "image_url".to_string(),
        );
        let mut item_repo_mock = MockItemRepository::new();
        item_repo_mock
            .expect_save()
            .times(1)
            .returning(|_| Box::pin(future::ready(Ok(()))));

        let handler = CreateItemHandler::new(Arc::new(item_repo_mock));

        let result = handler.handle(cmd).await;

        match result {
            Ok(_) => assert!(true),
            Err(_) => assert!(false, "should not return error when command is correct"),
        }
    }
}
