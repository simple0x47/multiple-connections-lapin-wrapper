use crate::config::amqp_input_api_consumer::AmqpInputApiConsumer;
use crate::config::amqp_output_api_consumer::AmqpOutputApiConsumer;
use crate::error::{Error, ErrorKind};
use cooplan_config_reader::reader::try_read;
use serde::{Deserialize, Serialize};

const API_CONSUMER_FILE: &str = "api_consumer.json";

#[derive(Deserialize, Serialize)]
pub struct ApiConsumer {
    input: Vec<AmqpInputApiConsumer>,
    output: Vec<AmqpOutputApiConsumer>,
}

impl ApiConsumer {
    pub fn input(&self) -> &[AmqpInputApiConsumer] {
        &self.input
    }

    pub fn output(&self) -> &[AmqpOutputApiConsumer] {
        &self.output
    }
}

pub async fn try_get() -> Result<ApiConsumer, Error> {
    match try_read(API_CONSUMER_FILE).await {
        Ok(api) => Ok(api),
        Err(error) => Err(Error::new(
            ErrorKind::InternalFailure,
            format!("failed to read api configuration: {}", error),
        )),
    }
}
