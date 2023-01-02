use lapin::options::BasicPublishOptions;
use lapin::types::FieldTable;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AmqpPublish {
    exchange: String,
    options: BasicPublishOptions,
    properties: FieldTable,
}

impl AmqpPublish {
    pub fn exchange(&self) -> &str {
        &self.exchange
    }

    pub fn options(&self) -> &BasicPublishOptions {
        &self.options
    }

    pub fn properties(&self) -> &FieldTable {
        &self.properties
    }
}
