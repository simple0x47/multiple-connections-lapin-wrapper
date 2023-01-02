use serde::{Deserialize, Serialize};
use lapin::options::{BasicAckOptions, BasicRejectOptions};
use crate::config::amqp_consume::AmqpConsume;
use crate::config::amqp_qos::AmqpQos;
use crate::config::amqp_queue::AmqpQueue;

#[derive(Deserialize, Serialize)]
pub struct AmqpQueueConsumer {
    queue: AmqpQueue,
    qos: AmqpQos,
    consume: AmqpConsume,
    acknowledge: BasicAckOptions,
    reject: BasicRejectOptions,
}
