use crate::config::amqp_queue_declare::AmqpQueueDeclare;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AmqpQueue {
    name: String,
    declare: AmqpQueueDeclare,
}
