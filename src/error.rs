use thiserror::Error;

#[derive(Error, Debug)]
pub enum RavenError {
    /// Input/output error
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Invalid SMTP command
    #[error("Invalid SMTP command: {0}")]
    InvalidCommand(String),

    /// Invalid email address
    #[error("Invalid email address: {0}")]
    InvalidEmailAddress(String),

    /// Connection error
    #[error("Connection error: {0}")]
    ConnectionError(String),

    /// SMTP protocol error
    #[error("SMTP protocol error: {0}")]
    ProtocolError(String),

    /// TLS error
    #[error("TLS error: {0}")]
    TlsError(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

pub type Result<T> = std::result::Result<T, RavenError>;
