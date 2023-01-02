use crate::config::amqp_queue_consumer::AmqpQueueConsumer;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AmqpInputApi {
    queue_consumer: AmqpQueueConsumer,
    max_concurrent_requests: u32,
}
