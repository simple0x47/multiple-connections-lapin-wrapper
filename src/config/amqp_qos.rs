use lapin::options::BasicQosOptions;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AmqpQos {
    prefetch_count: u32,
    options: BasicQosOptions,
}
