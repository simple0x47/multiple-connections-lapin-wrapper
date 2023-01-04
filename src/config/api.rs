use crate::config::amqp_input_api::AmqpInputApi;
use crate::config::amqp_output_api::AmqpOutputApi;
use crate::error::{Error, ErrorKind};
use cooplan_config_reader::reader::try_read;
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

pub async fn try_get(api_file: &str) -> Result<Api, Error> {
    match try_read(api_file).await {
        Ok(api) => Ok(api),
        Err(error) => Err(Error::new(
            ErrorKind::InternalFailure,
            format!("failed to read api configuration: {}", error),
        )),
    }
}
