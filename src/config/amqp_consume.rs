use lapin::options::BasicConsumeOptions;
use lapin::types::FieldTable;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AmqpConsume {
    options: BasicConsumeOptions,
    arguments: FieldTable,
}
