use crate::config::amqp_consume::AmqpConsume;
use crate::config::amqp_qos::AmqpQos;
use lapin::options::{BasicAckOptions, BasicRejectOptions};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AmqpOutputApiConsumer {
    queue_name: String,
    qos: AmqpQos,
    consume: AmqpConsume,
    acknowledge: BasicAckOptions,
    reject: BasicRejectOptions,
}
