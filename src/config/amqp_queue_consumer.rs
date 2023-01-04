use crate::config::amqp_consume::AmqpConsume;
use crate::config::amqp_qos::AmqpQos;
use crate::config::amqp_queue::AmqpQueue;
use lapin::options::{BasicAckOptions, BasicRejectOptions};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct AmqpQueueConsumer {
    queue: AmqpQueue,
    qos: AmqpQos,
    consume: AmqpConsume,
    acknowledge: BasicAckOptions,
    reject: BasicRejectOptions,
}

impl AmqpQueueConsumer {
    pub fn queue(&self) -> &AmqpQueue {
        &self.queue
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
}
