use lapin::options::BasicPublishOptions;
use lapin::types::FieldTable;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AmqpPublish {
    exchange: String,
    options: BasicPublishOptions,
    properties: FieldTable,
}
