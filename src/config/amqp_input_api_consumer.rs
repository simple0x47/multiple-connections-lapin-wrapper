use crate::config::amqp_queue_consumer::AmqpQueueConsumer;
use crate::config::amqp_request::AmqpRequest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct AmqpInputApiConsumer {
    id: String,
    request: AmqpRequest,
    response: AmqpQueueConsumer,
    wait_for_response_timeout_after_seconds: u64,
}

impl AmqpInputApiConsumer {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn request(&self) -> &AmqpRequest {
        &self.request
    }

    pub fn response(&self) -> &AmqpQueueConsumer {
        &self.response
    }

    pub fn wait_for_response_timeout_after_seconds(&self) -> u64 {
        self.wait_for_response_timeout_after_seconds
    }
}
