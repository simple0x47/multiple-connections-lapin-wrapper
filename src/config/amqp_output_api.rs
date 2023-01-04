use crate::config::amqp_publish::AmqpPublish;
use crate::config::amqp_queue::AmqpQueue;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct AmqpOutputApi {
    id: String,
    queue: AmqpQueue,
    publish: AmqpPublish,
}

impl AmqpOutputApi {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn queue(&self) -> &AmqpQueue {
        &self.queue
    }

    pub fn publish(&self) -> &AmqpPublish {
        &self.publish
    }
}
