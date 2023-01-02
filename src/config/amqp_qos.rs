use lapin::options::BasicQosOptions;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AmqpQos {
    prefetch_count: u32,
    options: BasicQosOptions,
}

impl AmqpQos {
    pub fn prefetch_count(&self) -> u32 {
        self.prefetch_count
    }

    pub fn options(&self) -> &BasicQosOptions {
        &self.options
    }
}
