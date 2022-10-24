use super::event::Event;

pub struct ItemCreated {
    pub id: String,
    pub payload: String,
    pub version: u16,
}

impl ItemCreated {
    pub fn new(id: String, payload: String, version: u16) -> ItemCreated {
        ItemCreated {
            id,
            payload,
            version,
        }
    }
}

impl Event for ItemCreated {
    fn name(&self) -> &str {
        "ItemCreated"
    }

    fn payload(&self) -> &str {
        self.payload.as_str()
    }

    fn version(&self) -> u16 {
        self.version
    }

    fn id(&self) -> &str {
        self.id.as_str()
    }
}
