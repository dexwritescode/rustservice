use config::{Config, ConfigError, Environment, File};
//use lazy_static::lazy_static;
use serde::Deserialize;
use std::{env, fmt};

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Logger {
    pub level: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub host: String,
    pub name: String,
    pub user: String,
    pub password: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Configurations {
    pub environment: String,
    pub server: Server,
    pub logger: Logger,
    pub database: Database,
}

impl Configurations {
    pub fn new() -> Result<Self, ConfigError> {
        let env = env::var("ENV").unwrap_or_else(|_| "development".into());

        let mut builder = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{env}")).required(false))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::default().separator("__"));

        // Allow to override settings from environment variables
        if let Ok(port) = env::var("PORT") {
            builder = builder.set_override("server.port", port)?;
        }
        if let Ok(log_level) = env::var("LOG_LEVEL") {
            builder = builder.set_override("logger.level", log_level)?;
        }

        builder
            .build()?
            // Deserialize (and thus freeze) the entire configuration.
            .try_deserialize()
    }
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "http://localhost:{}", &self.port)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load_prod_config() {
        temp_env::with_vars([("ENV", Some("production"))], || {
            let actual = Configurations::new().unwrap();
            assert_eq!(actual.environment, "production");
            assert_eq!(actual.logger.level, "info");
            assert_eq!(actual.server.host, "localhost");
            assert_eq!(actual.server.port, 8080);
            assert_eq!(actual.database.host, "localhost");
            assert_eq!(actual.database.port, 5432);
            assert_eq!(actual.database.name, "rustservice");
            assert_eq!(actual.database.user, "rustservice");
            assert_eq!(actual.database.password, "rustservice");
        });
    }

    #[test]
    fn test_override_port() {
        temp_env::with_vars([("PORT", Some("8899"))], || {
            let actual = Configurations::new().unwrap();
            assert_eq!(actual.environment, "development");
            assert_eq!(actual.server.port, 8899);
            assert_eq!(actual.logger.level, "debug");
        });
    }
}
