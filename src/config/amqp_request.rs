use crate::config::amqp_publish::AmqpPublish;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct AmqpRequest {
    queue_name: String,
    publish: AmqpPublish,
}

impl AmqpRequest {
    pub fn queue_name(&self) -> &str {
        &self.queue_name
    }

    pub fn publish(&self) -> &AmqpPublish {
        &self.publish
    }
}
