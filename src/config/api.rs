use crate::config::amqp_input_api::AmqpInputApi;
use crate::config::amqp_output_api::AmqpOutputApi;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Api {
    input: Vec<AmqpInputApi>,
    output: Vec<AmqpOutputApi>,
}

impl Api {
    pub fn input(&self) -> &[AmqpInputApi] {
        &self.input
    }

    pub fn output(&self) -> &[AmqpOutputApi] {
        &self.output
    }
}
