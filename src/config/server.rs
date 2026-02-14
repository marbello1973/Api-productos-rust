use std::{env, net::SocketAddr};

use crate::error::ConfigError;

pub trait ServerAddress {
    fn address(&self) -> String;
    fn url(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    host: String,
    port: u16,
}

impl ServerAddress for ServerConfig {
    fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
    fn url(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }
}

impl ServerConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv::dotenv().ok();

        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        let port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .map_err(|e| ConfigError::InvalidPort(format!("error: {:?}", e).to_string()))?;

        Ok(Self { host, port })
    }

    pub fn socket_addr(&self) -> Result<SocketAddr, ConfigError> {
        match format!("{}:{}", self.host, self.port).parse() {
            Ok(addr) => Ok(addr),
            Err(error) => Err(ConfigError::InvalidPort(
                format!("Error: {}", error).to_string(),
            )),
        }
    }

    pub fn url(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }
}
