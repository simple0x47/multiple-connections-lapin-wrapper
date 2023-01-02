use crate::config::amqp_publish::AmqpPublish;
use crate::config::amqp_queue_consumer::AmqpQueueConsumer;
use crate::config::amqp_request::AmqpRequest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AmqpInputApiConsumer {
    request: AmqpRequest,
    response: AmqpQueueConsumer,
}

impl AmqpInputApiConsumer {
    pub fn request(&self) -> &AmqpRequest {
        &self.request
    }

    pub fn response(&self) -> &AmqpQueueConsumer {
        &self.response
    }
}
