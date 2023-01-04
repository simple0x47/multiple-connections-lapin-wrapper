use crate::config::amqp_input_api_consumer::AmqpInputApiConsumer;
use crate::config::amqp_output_api_consumer::AmqpOutputApiConsumer;
use crate::error::{Error, ErrorKind};
use cooplan_config_reader::reader::try_read;
use serde::{Deserialize, Serialize};

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

pub async fn try_get(api_consumer_file: &str) -> Result<ApiConsumer, Error> {
    match try_read(api_consumer_file).await {
        Ok(api) => Ok(api),
        Err(error) => Err(Error::new(
            ErrorKind::InternalFailure,
            format!("failed to read api configuration: {}", error),
        )),
    }
}
