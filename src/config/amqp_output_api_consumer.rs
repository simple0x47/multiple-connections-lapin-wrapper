use crate::config::amqp_consume::AmqpConsume;
use crate::config::amqp_qos::AmqpQos;
use lapin::options::{BasicAckOptions, BasicRejectOptions};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct AmqpOutputApiConsumer {
    id: String,
    queue_name: String,
    qos: AmqpQos,
    consume: AmqpConsume,
    acknowledge: BasicAckOptions,
    reject: BasicRejectOptions,
    timeout_after_seconds: u64,
}

impl AmqpOutputApiConsumer {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn queue_name(&self) -> &str {
        &self.queue_name
    }

    pub fn qos(&self) -> &AmqpQos {
        &self.qos
    }

    pub fn consume(&self) -> &AmqpConsume {
        &self.consume
    }

    pub fn acknowledge(&self) -> &BasicAckOptions {
        &self.acknowledge
    }

    pub fn reject(&self) -> &BasicRejectOptions {
        &self.reject
    }
    
    pub fn timeout_after_seconds(&self) -> u64 {
        self.timeout_after_seconds
    }
}
