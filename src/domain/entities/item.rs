use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: BigDecimal,
    pub image_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Item {
    pub fn new(
        id: String,
        name: String,
        description: String,
        price: BigDecimal,
        image_url: String,
    ) -> Item {
        Item {
            id,
            name,
            description,
            price,
            image_url,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
