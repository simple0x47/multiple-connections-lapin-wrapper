use lapin::options::QueueDeclareOptions;
use lapin::types::FieldTable;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AmqpQueueDeclare {
    pub options: QueueDeclareOptions,
    pub arguments: FieldTable,
}
