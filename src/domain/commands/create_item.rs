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
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    use bigdecimal::{BigDecimal, FromPrimitive};

    use crate::{
        domain::{commands::Command, entities::Item},
        infra::repositories::InMemoryItemRepository,
    };

    use super::{CreateItem, CreateItemHandler};

    #[test]
    fn it_should_create_command() {
        let expected = CreateItem {
            name: "name".to_string(),
            description: "description".to_string(),
            price: BigDecimal::from_f32(1.0).unwrap(),
            image_url: "image_url".to_string(),
        };

        let cmd = CreateItem::new(
            expected.name.clone(),
            expected.description.clone(),
            expected.price.clone(),
            expected.image_url.clone(),
        );

        assert_eq!(expected, cmd)
    }

    #[tokio::test]
    async fn it_should_handle_command() {
        let cmd = CreateItem::new(
            "name".to_string(),
            "description".to_string(),
            BigDecimal::from_f32(1.0).unwrap(),
            "image_url".to_string(),
        );
        let item_store = Arc::new(Mutex::new(HashMap::<String, Item>::new()));
        let item_repo = Arc::new(InMemoryItemRepository::new(item_store));
        let handler = CreateItemHandler::new(item_repo);

        let result = handler.handle(cmd).await;

        match result {
            Ok(_) => assert!(true),
            Err(_) => assert!(false, "should not return error when command is correct"),
        }
    }
}
