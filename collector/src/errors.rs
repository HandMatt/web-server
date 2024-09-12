use thiserror::Error;

/// Errors that can occur when interacting with the data collector.
#[derive(Debug, Error)]
pub enum CollectorError {
    #[error("Unable to connect to the server")]
    UnableToConnect,
    #[error("Unable to send data to the server")]
    UnableToSendData,
}
