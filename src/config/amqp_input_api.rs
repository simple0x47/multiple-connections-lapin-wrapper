use crate::config::amqp_queue_consumer::AmqpQueueConsumer;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AmqpInputApi {
    queue_consumer: AmqpQueueConsumer,
    max_concurrent_requests: u32,
}

impl AmqpInputApi {
    pub fn queue_consumer(&self) -> &AmqpQueueConsumer {
        &self.queue_consumer
    }

    pub fn max_concurrent_requests(&self) -> u32 {
        self.max_concurrent_requests
    }
}
