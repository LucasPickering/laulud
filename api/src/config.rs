use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

pub const DATABASE_NAME: &str = "laulud";

/// App-wide configuration settings
#[derive(Debug, Deserialize)]
pub struct LauludConfig {
    /// The URL of the DB that we connect to, as a Mongo URI.
    /// https://docs.mongodb.com/manual/reference/connection-string/
    pub database_url: String,
    /// The hostname for the HTTP server to bind to.
    pub server_host: String,
}

impl LauludConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Load "config/default.json", which has non-sensitive values
        s.merge(File::with_name("config/default.json"))?;
        // Optionally load a "config/dev.json" file
        s.merge(File::with_name("config/dev").required(false))?;
        // Load all env variables with the prefix "LAULUD_"
        s.merge(Environment::new().prefix("laulud").separator("__"))?;

        s.try_into()
    }
}
