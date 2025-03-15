use crate::error::{RavenError, Result};
use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
    time::Duration,
};

use log::{debug, info};

use crate::config::ClientConfig;

/// SMTP response codes
pub mod response_code {
    pub const READY: &str = "220";
    pub const OK: &str = "250";
    pub const DATA_START: &str = "354";
    pub const BYE: &str = "221";
}

/// States of the SMTP client
#[derive(Debug, PartialEq)]
enum SmtpState {
    Initial,
    Connected,
    Greeted,
    MailFrom,
    RcptTo,
    Data,
    Quit,
}

/// SMTP client that can send emails
pub struct SmtpClient {
    config: ClientConfig,
    state: SmtpState,
    stream: Option<TcpStream>,
}

impl SmtpClient {
    /// Create a new SMTP client with the given configuration
    pub fn new(config: ClientConfig) -> Self {
        SmtpClient {
            config,
            state: SmtpState::Initial,
            stream: None,
        }
    }

    /// Connect to the SMTP server
    pub fn connect(&mut self) -> Result<()> {
        info!(
            "Connecting to SMTP server {}:{}...",
            self.config.server, self.config.port
        );

        let addr = format!("{}:{}", self.config.server, self.config.port);
        let stream = TcpStream::connect_timeout(
            &addr.parse().map_err(|e: std::net::AddrParseError| {
                RavenError::ConnectionError(e.to_string())
            })?,
            Duration::from_secs(self.config.timeout),
        )?;
        stream.set_read_timeout(Some(Duration::from_secs(self.config.timeout)))?;
        stream.set_write_timeout(Some(Duration::from_secs(self.config.timeout)))?;

        self.stream = Some(stream);
        self.state = SmtpState::Connected;

        // Read the server's greeting
        let response = self.read_response()?;
        if !response.starts_with(response_code::READY) {
            return Err(RavenError::ProtocolError(format!(
                "Unexpected server greeting: {}",
                response
            )));
        }

        info!("Connected to SMTP server");
        Ok(())
    }

    /// Read a response from the SMTP server
    fn read_response(&mut self) -> Result<String> {
        if let Some(stream) = &mut self.stream {
            let mut reader = BufReader::new(stream);
            let mut line = String::new();
            reader.read_line(&mut line)?;

            debug!("Recieved response: {}", line.trim());
            Ok(line)
        } else {
            Err(RavenError::ConnectionError("Not connected".to_string()))
        }
    }
}
