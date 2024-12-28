use dotenv::dotenv;
use std::env;
use thiserror::Error;

// ! fixme: the `dotenv` package hasn't been updated for a long time,
//          consider switching to other similar packages.

#[derive(Error, Debug)]
pub enum SettingsError {
    #[error("Missed env: {0}")]
    MissedEnv(#[from] env::VarError),

    #[error("Invalid port: {0}")]
    InvalidPort(#[from] std::num::ParseIntError),
}

pub struct Settings {
    host: String,
    port: u16,
    db: String,
}

impl Settings {
    pub fn new() -> Result<Self, SettingsError> {
        dotenv().ok();

        Ok(Self {
            host: env::var("ATARIGO_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("ATARIGO_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?,
            db: env::var("ATARIGO_DB").unwrap_or_else(|_| "sqlite://sqlite.db".to_string()),
        })
    }

    pub fn server_addr(&self) -> (&str, u16) {
        (&self.host, self.port)
    }

    pub fn db_addr(&self) -> &str {
        &self.db
    }
}
