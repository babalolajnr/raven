use std::{net::SocketAddr, path::PathBuf};

#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Server address to connect to.
    pub server: String,
    /// Server port
    pub port: u16,
    /// Sender email address
    pub from: String,
    /// Recipient email addresses
    pub recipients: Vec<String>,
    /// Email subject
    pub subject: String,
    /// Email body
    pub body: String,
    /// Connection timout in seconds
    pub timeout: u64,
    /// Whether to use TLS for the connection
    pub use_tls: bool,
    /// Custom TLS certificate path
    pub tls_cert_path: Option<PathBuf>,
}

impl Default for ClientConfig {
    fn default() -> Self {
        ClientConfig {
            server: "127.0.0.1".to_string(),
            port: 2525,
            from: "sender@example.com".to_string(),
            recipients: vec!["recipient@example.com".to_string()],
            subject: "Test Email".to_string(),
            body: "This is a test email.".to_string(),
            timeout: 30,
            use_tls: false,
            tls_cert_path: None,
        }
    }
}

/// Configuration for the SMTP server.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Address to bind the server to
    pub bind_addr: SocketAddr,
    /// Whether to use TLS
    pub use_tls: bool,
    /// Path to the TLS certificate
    pub tls_cert_path: Option<PathBuf>,
    /// Path to the TLS key
    pub tls_key_path: Option<PathBuf>,
    /// Maximum message size in bytes
    pub max_message_size: usize,
    /// Connection timeout in seconds
    pub timeout: u64,
    /// Maximum number of recipients per email
    pub max_recipients: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            bind_addr: "127.0.0.1:2525".parse().unwrap(),
            use_tls: false,
            tls_cert_path: None,
            tls_key_path: None,
            max_message_size: 10 * 1024 * 1024, // 10MB
            timeout: 60,
            max_recipients: 100,
        }
    }
}
