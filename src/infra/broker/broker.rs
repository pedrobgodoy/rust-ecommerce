#[cfg(test)]
use mockall::{automock, predicate::*};

use async_trait::async_trait;
use lapin::{
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties,
};

use crate::domain::events::event::Event;

#[async_trait]
#[cfg_attr(test, automock)]
pub trait Broker {
    async fn publish(&self, event: Box<dyn Event + Send + Sync>) -> Result<(), ()>;
}

pub struct AMQPBroker {
    channel: lapin::Channel,
}

impl AMQPBroker {
    pub async fn new() -> Self {
        let connection = lapin::Connection::connect(
            "amqp://guest:guest@localhost:5672/%2f",
            lapin::ConnectionProperties::default(),
        )
        .await
        .unwrap();
        let channel = connection.create_channel().await.unwrap();
        channel
            .queue_declare(
                "catalog",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();
        AMQPBroker { channel }
    }
}

#[async_trait]
impl Broker for AMQPBroker {
    async fn publish(&self, event: Box<dyn Event + Send + Sync>) -> Result<(), ()> {
        self.channel
            .basic_publish(
                "",
                "catalog",
                BasicPublishOptions::default(),
                event.payload().as_bytes(),
                BasicProperties::default(),
            )
            .await
            .unwrap();
        Ok(())
    }
}
