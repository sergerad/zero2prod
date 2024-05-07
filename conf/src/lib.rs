use secrecy::{ExposeSecret, Secret};

use sqlx::postgres::{PgConnectOptions, PgSslMode};

use serde_aux::field_attributes::deserialize_number_from_string;

use domain::SubscriberEmail;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub email_client: EmailClientSettings,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub base_url: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct EmailClientSettings {
    pub base_url: String,
    pub sender_email: String,
    #[serde(serialize_with = "serialize_secret")]
    pub authorization_token: Secret<String>,
    pub timeout_milliseconds: u64,
}

impl EmailClientSettings {
    pub fn sender(&self) -> Result<domain::SubscriberEmail, String> {
        SubscriberEmail::parse(self.sender_email.clone())
    }

    pub fn timeout(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.timeout_milliseconds)
    }
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl From<String> for Environment {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "production" => Self::Production,
            _ => Self::Local,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    #[serde(serialize_with = "serialize_secret")]
    pub password: Secret<String>,
    pub host: String,
    pub port: u16,
    pub database_name: String,
    pub require_ssl: bool,
}

fn serialize_secret<S>(x: &Secret<String>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    s.serialize_str(x.expose_secret())
}

impl DatabaseSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
    }

    pub fn connection_string(&self, port: u16) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            port,
            self.database_name,
        ))
    }
}

#[derive(serde::Deserialize)]
pub struct EnvFile {
    pub database_url: Secret<String>,
}

pub fn config_dir(relative_offset: Option<String>) -> anyhow::Result<std::path::PathBuf> {
    if let Some(offset) = relative_offset {
        return Ok(std::env::current_dir()?.join(offset).join("conf"));
    }
    Ok(std::env::current_dir()?.join("conf"))
}

pub fn base_config_file() -> anyhow::Result<std::path::PathBuf> {
    Ok(config_dir(None)?.join("base.yaml"))
}

pub fn get_configuration(relative_offset: Option<String>) -> anyhow::Result<Settings> {
    // Config dir path
    let config_dir = config_dir(relative_offset)?;

    // Determine config file based on env
    let env: Environment = std::env::var("ZERO2PROD_ENV")
        .unwrap_or_else(|_| "local".into())
        .into();
    let env_filename = format!("{}.yaml", env.as_str());

    // Build configuration
    let config = config::Config::builder()
        .add_source(config::File::from(config_dir.join("base.yaml")))
        .add_source(config::File::from(config_dir.join(env_filename)))
        .add_source(
            config::Environment::with_prefix("ZERO2PROD")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    // Return settings
    let settings = config.try_deserialize::<Settings>()?;
    Ok(settings)
}
