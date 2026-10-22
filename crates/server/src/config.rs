use std::path::PathBuf;

pub use config_load::config::ConfigError;
use config_load::config::builder::DefaultState;
use config_load::config::{ConfigBuilder, Environment};
use config_load::{ConfigLoader, FileLocation, Load};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct HttpConfig {
    #[serde(default = "HttpConfig::default_host")]
    pub host: String,

    #[serde(default = "HttpConfig::default_port")]
    pub port: u16,

    /// Каталог статики, собранной `cargo make client`.
    #[serde(default = "HttpConfig::default_webassets_dir")]
    pub webassets_dir: PathBuf,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            host: Self::default_host(),
            port: Self::default_port(),
            webassets_dir: Self::default_webassets_dir(),
        }
    }
}

impl HttpConfig {
    pub fn default_host() -> String {
        "127.0.0.1".into()
    }

    pub const fn default_port() -> u16 {
        8080
    }

    pub fn default_webassets_dir() -> PathBuf {
        "target/web".into()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct DatabaseConfig {
    #[serde(default = "DatabaseConfig::default_url")]
    pub url: String,

    #[serde(default = "DatabaseConfig::default_max_connections")]
    pub max_connections: u32,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: Self::default_url(),
            max_connections: Self::default_max_connections(),
        }
    }
}

impl DatabaseConfig {
    /// Postgres из `docker-compose.yml`.
    pub fn default_url() -> String {
        "postgres://freemax:freemax@localhost:5432/freemax".into()
    }

    pub const fn default_max_connections() -> u32 {
        10
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct LoggerConfig {
    /// Фильтр `tracing` (синтаксис `RUST_LOG`); `RUST_LOG` имеет приоритет.
    #[serde(default = "LoggerConfig::default_filter")]
    pub filter: String,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            filter: Self::default_filter(),
        }
    }
}

impl LoggerConfig {
    pub fn default_filter() -> String {
        "info".into()
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    pub http: HttpConfig,
    pub database: DatabaseConfig,
    pub log: LoggerConfig,
}

impl Config {
    /// Порядок источников (каждый следующий перекрывает предыдущий):
    /// файл из `FREEMAX_ROOT_CONFIG`, файл из `--config` или первый `config.toml` в текущем и родительских
    /// каталогах, переменные окружения `FREEMAX_*`.
    pub fn load(config_file: Option<PathBuf>) -> config_load::Result<Self> {
        ConfigLoader::default()
            .add(FileLocation::first_some_path().from_env("FREEMAX_ROOT_CONFIG"))
            .exclude_not_exists()
            .add(
                FileLocation::first_some_path()
                    .from_file(config_file)
                    .from_cwd_and_parents_exists("config.toml"),
            )
            .load()
    }
}

impl Load for Config {
    fn load(config_builder: ConfigBuilder<DefaultState>) -> config_load::Result<Self> {
        // Секции разделяются `__`, чтобы ключи могли содержать `_`.
        // Например, `FREEMAX_DATABASE__MAX_CONNECTIONS=20` задаёт `database.max_connections`.
        let config = config_builder
            .add_source(
                Environment::with_prefix("FREEMAX")
                    .prefix_separator("_")
                    .separator("__")
                    .try_parsing(true),
            )
            .build()?;
        config.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_config_matches_defaults() {
        let path = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../config.example.toml"));
        let config = Config::load(Some(path)).unwrap();
        let default = Config::default();

        assert_eq!(config.http.host, default.http.host);
        assert_eq!(config.http.port, default.http.port);
        assert_eq!(config.http.webassets_dir, default.http.webassets_dir);
        assert_eq!(config.database.url, default.database.url);
        assert_eq!(config.database.max_connections, default.database.max_connections);
    }
}
