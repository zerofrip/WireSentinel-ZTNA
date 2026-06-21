use thiserror::Error;

#[derive(Debug, Error)]
pub enum ZtnaError {
    #[error("identity error: {0}")]
    Identity(String),

    #[error("trust error: {0}")]
    Trust(String),

    #[error("policy error: {0}")]
    Policy(String),

    #[error("gateway error: {0}")]
    Gateway(String),

    #[error("connector error: {0}")]
    Connector(String),

    #[error("segmentation error: {0}")]
    Segmentation(String),

    #[error("publishing error: {0}")]
    Publishing(String),

    #[error("configuration error: {0}")]
    Config(String),

    #[error("{0}")]
    Other(String),
}

pub type ZtnaResult<T> = std::result::Result<T, ZtnaError>;

impl From<shared_types::WireSentinelError> for ZtnaError {
    fn from(value: shared_types::WireSentinelError) -> Self {
        Self::Other(value.to_string())
    }
}
