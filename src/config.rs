use std::env;

const VALID_LOG_LEVELS: &[&str] = &["trace", "debug", "info", "warn", "error", "off"];

fn get_log_level() -> String {
    let level = env::var("LOG_LEVEL")
        .unwrap_or_else(|_| "info".to_string())
        .to_lowercase();

    if VALID_LOG_LEVELS.contains(&level.as_str()) {
        level
    } else {
        "info".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub log_level: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
            log_level: get_log_level(),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 3000,
            log_level: "info".to_string(),
            database_url: "postgresql://postgres:postgres@localhost:5432/warehouse_db".to_string(),
        }
    }
}
