use lapin::{
    tcp::{OwnedIdentity, OwnedTLSConfig},
    types::FieldTable,
    ConnectionProperties,
};
use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer,
};
use serde_json::{Map, Value};

use crate::error::{Error, ErrorKind};

const URI_KEY: &str = "uri";
const OPTIONS_KEY: &str = "options";
const LOCALE_KEY: &str = "locale";
const CLIENT_PROPERTIES_KEY: &str = "client_properties";
const OWNED_TLS_CONFIG_KEY: &str = "owned_tls_config";
const IDENTITY_KEY: &str = "identity";
const DER_ENCODED_IDENTITY_KEY: &str = "der";
const DECRYPTION_PASSWORD_KEY: &str = "password";
const CERTIFICATES_CHAIN_KEY: &str = "cert_chain";

const EXPECTED_FIELDS: &'static [&'static str] = &[URI_KEY, OPTIONS_KEY, OWNED_TLS_CONFIG_KEY];

pub struct AmqpConnectConfig {
    uri: String,
    options: ConnectionProperties,
    owned_tls_config: OwnedTLSConfig,
}

impl AmqpConnectConfig {
    pub fn new(
        uri: String,
        options: ConnectionProperties,
        owned_tls_config: OwnedTLSConfig,
    ) -> AmqpConnectConfig {
        AmqpConnectConfig {
            uri,
            options,
            owned_tls_config,
        }
    }

    pub fn uri(&self) -> &str {
        self.uri.as_str()
    }

    pub fn cloned_options(&self) -> ConnectionProperties {
        self.options.clone()
    }

    pub fn cloned_owned_tls_config(&self) -> OwnedTLSConfig {
        let identity = match &self.owned_tls_config.identity {
            Some(identity) => {
                let der = identity.der.clone();
                let password = identity.password.clone();

                Some(OwnedIdentity { der, password })
            }
            None => None,
        };

        let cert_chain = self.owned_tls_config.cert_chain.clone();

        OwnedTLSConfig {
            identity,
            cert_chain,
        }
    }
}

fn get_connection_properties_from_json_object(
    mut object: Map<String, Value>,
) -> Result<ConnectionProperties, Error> {
    // This wrapper is intented to work ***ONLY WITH TOKIO*** as the runtime.
    let mut properties = ConnectionProperties::default()
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio);

    properties.locale = match object.remove(LOCALE_KEY) {
        Some(value) => match value.as_str() {
            Some(locale) => locale.to_string(),
            None => {
                return Err(Error::new(
                    ErrorKind::ConfigFailure,
                    "failed to deserialize locale as string",
                ))
            }
        },
        None => {
            return Err(Error::new(
                ErrorKind::ConfigFailure,
                format!("'{}' is missing from config", LOCALE_KEY),
            ))
        }
    };

    properties.client_properties = match object.remove(CLIENT_PROPERTIES_KEY) {
        Some(value) => match serde_json::from_value::<FieldTable>(value) {
            Ok(client_properties) => client_properties,
            Err(error) => {
                return Err(Error::new(
                    ErrorKind::ConfigFailure,
                    format!("failed to deserialize connection properties: {}", error),
                ))
            }
        },
        None => {
            return Err(Error::new(
                ErrorKind::ConfigFailure,
                format!("'{}' is missing from config", CLIENT_PROPERTIES_KEY),
            ))
        }
    };

    Ok(properties)
}

fn get_identity_from_json_object(
    mut object: &mut Map<String, Value>,
) -> Result<OwnedIdentity, Error> {
    let encoded_identity = match object.remove(DER_ENCODED_IDENTITY_KEY) {
        Some(value) => match serde_json::from_value::<Vec<u8>>(value) {
            Ok(encoded_identity) => encoded_identity,
            Err(error) => {
                return Err(Error::new(
                    ErrorKind::ConfigFailure,
                    format!(
                        "failed to read '{}' as a vector of bytes: {}",
                        DER_ENCODED_IDENTITY_KEY, error
                    ),
                ))
            }
        },
        None => {
            return Err(Error::new(
                ErrorKind::ConfigFailure,
                format!(
                    "missing '{}' from identity's config",
                    DER_ENCODED_IDENTITY_KEY
                ),
            ))
        }
    };

    let password = match object.remove(DECRYPTION_PASSWORD_KEY) {
        Some(value) => match serde_json::from_value::<String>(value) {
            Ok(password) => password,
            Err(error) => {
                return Err(Error::new(
                    ErrorKind::ConfigFailure,
                    format!(
                        "failed to deserialize '{}' as a String: {}",
                        DECRYPTION_PASSWORD_KEY, error
                    ),
                ))
            }
        },
        None => {
            return Err(Error::new(
                ErrorKind::ConfigFailure,
                format!(
                    "missing '{}' from identity's config",
                    DECRYPTION_PASSWORD_KEY
                ),
            ))
        }
    };

    let owned_identity = OwnedIdentity {
        der: encoded_identity,
        password,
    };

    Ok(owned_identity)
}

fn get_owned_tls_config_from_json_object(
    mut object: Map<String, Value>,
) -> Result<OwnedTLSConfig, Error> {
    let mut json_identity = match object.remove(IDENTITY_KEY) {
        Some(value) => match serde_json::from_value::<Map<String, Value>>(value) {
            Ok(json_identity) => Some(json_identity),
            Err(error) => {
                return Err(Error::new(
                    ErrorKind::ConfigFailure,
                    format!(
                        "failed to deserialize '{}' as JSON object: {}",
                        IDENTITY_KEY, error
                    ),
                ))
            }
        },
        None => None,
    };

    let identity = match json_identity {
        Some(mut json_identity) => Some(get_identity_from_json_object(&mut json_identity)?),
        None => None,
    };

    let certificates_chain = match object.remove(CERTIFICATES_CHAIN_KEY) {
        Some(value) => match serde_json::from_value::<String>(value) {
            Ok(certificates_chain) => Some(certificates_chain),
            Err(error) => {
                return Err(Error::new(
                    ErrorKind::ConfigFailure,
                    format!(
                        "failed to deserialize '{}' as a String: {}",
                        CERTIFICATES_CHAIN_KEY, error
                    ),
                ))
            }
        },
        None => None,
    };

    let owned_tls_config = OwnedTLSConfig {
        identity,
        cert_chain: certificates_chain,
    };

    Ok(owned_tls_config)
}

impl<'de> Deserialize<'de> for AmqpConnectConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            Uri,
            Options,
            OwnedTlsConfig,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("'uri', 'options' or 'owned_tls_config")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "uri" => Ok(Field::Uri),
                            "options" => Ok(Field::Options),
                            "owned_tls_config" => Ok(Field::OwnedTlsConfig),
                            _ => Err(serde::de::Error::unknown_field(value, EXPECTED_FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct AmqpConnectConfigVisitor;

        impl<'de> Visitor<'de> for AmqpConnectConfigVisitor {
            type Value = AmqpConnectConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct AmqpConnectConfigVisitor")
            }

            fn visit_map<V>(self, mut map: V) -> Result<AmqpConnectConfig, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut uri = None;
                let mut options = None;
                let mut owned_tls_config = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Uri => {
                            if uri.is_some() {
                                return Err(serde::de::Error::duplicate_field(URI_KEY));
                            }

                            uri = Some(map.next_value()?);
                        }
                        Field::Options => {
                            if options.is_some() {
                                return Err(serde::de::Error::duplicate_field(OPTIONS_KEY));
                            }

                            let json_options = map.next_value::<Map<String, Value>>()?;

                            options = match get_connection_properties_from_json_object(json_options)
                            {
                                Ok(options) => Some(options),
                                Err(error) => return Err(serde::de::Error::custom(error.message)),
                            };
                        }
                        Field::OwnedTlsConfig => {
                            if owned_tls_config.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    OWNED_TLS_CONFIG_KEY,
                                ));
                            }

                            let json_owned_tls_config = map.next_value::<Map<String, Value>>()?;

                            owned_tls_config = match get_owned_tls_config_from_json_object(
                                json_owned_tls_config,
                            ) {
                                Ok(owned_tls_config) => Some(owned_tls_config),
                                Err(error) => return Err(serde::de::Error::custom(error.message)),
                            };
                        }
                    }
                }

                let uri = uri.ok_or_else(|| serde::de::Error::missing_field(URI_KEY))?;
                let options =
                    options.ok_or_else(|| serde::de::Error::missing_field(OPTIONS_KEY))?;
                let owned_tls_config = owned_tls_config
                    .ok_or_else(|| serde::de::Error::missing_field(OWNED_TLS_CONFIG_KEY))?;

                Ok(AmqpConnectConfig {
                    uri,
                    options,
                    owned_tls_config,
                })
            }
        }

        deserializer.deserialize_struct(
            "AmqpConnectConfig",
            EXPECTED_FIELDS,
            AmqpConnectConfigVisitor,
        )
    }
}

#[cfg(test)]
#[tokio::test]
async fn deserialize_correctly_from_without_owned_tls_config() {
    let json: &str = r#"{
                            "uri": "amqp://guest:guest@127.0.0.1:5672",
                            "options": {
                                "locale": "en_US",
                                "client_properties": {}
                            },
                            "owned_tls_config": {}
                        }"#;

    let config = match serde_json::from_str::<AmqpConnectConfig>(json) {
        Ok(config) => config,
        Err(error) => panic!("failed to deserialize amqp connect config: {}", error),
    };
}

#[tokio::test]
async fn deserialize_correctly_from_complete_config() {
    let json: &str = r#"{
                            "uri": "amqp://guest:guest@127.0.0.1:5672",
                            "options": {
                                "locale": "en_US",
                                "client_properties": {
                                }
                            },
                            "owned_tls_config": {
                                "identity": {
                                    "der": [128, 130, 244, 139, 28, 0],
                                    "password": "abcdTEST1234"
                                },
                                "cert_chain": "ijsfioqwjfoqbfwioqufqfiuwqjfoiqwjfioqjwfiojqoi"
                            }
                        }"#;

    let config = match serde_json::from_str::<AmqpConnectConfig>(json) {
        Ok(config) => config,
        Err(error) => panic!("failed to deserialize amqp connect config: {}", error),
    };

    let uri = config.uri;
    let options = config.options;
    let identity = config.owned_tls_config.identity.unwrap();

    assert_eq!(&uri, "amqp://guest:guest@127.0.0.1:5672");

    assert_eq!(&identity.der, &vec![128u8, 130u8, 244u8, 139u8, 28u8, 0u8]);
    assert_eq!(&identity.password, "abcdTEST1234");
    assert_eq!(
        config.owned_tls_config.cert_chain.unwrap(),
        "ijsfioqwjfoqbfwioqufqfiuwqjfoiqwjfioqjwfiojqoi"
    )
}
