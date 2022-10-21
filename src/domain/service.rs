use std::sync::Arc;

use super::{commands, queries, repositories::ItemRepository};

pub struct ApplicationService {
    // Command handlers
    pub create_item: commands::CreateItemHandler,

    // Query handlers
    pub get_item: queries::GetItemHandler,
}

impl ApplicationService {
    pub fn new(item_repo: Arc<dyn ItemRepository + Send + Sync>) -> Self {
        Self {
            create_item: commands::CreateItemHandler::new(item_repo.clone()),
            get_item: queries::GetItemHandler::new(item_repo.clone()),
        }
    }
}
