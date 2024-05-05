use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub database: pg::DatabaseSettings,
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

pub fn get_configuration() -> anyhow::Result<Settings> {
    // Config dir path
    let base_path = std::env::current_dir()?;
    let config_dir = base_path.join("configuration");

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
