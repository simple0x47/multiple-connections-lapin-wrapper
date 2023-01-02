use crate::config::amqp_publish::AmqpPublish;
use crate::config::amqp_queue::AmqpQueue;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AmqpOutputApi {
    queue: AmqpQueue,
    publish: AmqpPublish,
}
