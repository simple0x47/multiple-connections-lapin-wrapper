use crate::config::amqp_publish::AmqpPublish;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AmqpRequest {
    queue_name: String,
    publish: AmqpPublish,
}
