use crate::config::amqp_queue_declare::AmqpQueueDeclare;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct AmqpQueue {
    name: String,
    declare: AmqpQueueDeclare,
}

impl AmqpQueue {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn declare(&self) -> &AmqpQueueDeclare {
        &self.declare
    }
}
