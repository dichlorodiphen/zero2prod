use config::{Config, File, FileFormat};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    /// Produces an sqlx PgConnection connection string from the settings.
    pub fn connection_string(&self) -> String {
        let user = &self.username;
        let password = &self.password;
        let host = &self.host;
        let port = &self.port;
        let dbname = &self.database_name;

        format!("postgresql://{user}:{password}@{host}:{port}/{dbname}")
    }

    /// Produces a Postgres connection string without the database name.
    pub fn connection_string_without_db(&self) -> String {
        let user = &self.username;
        let password = &self.password;
        let host = &self.host;
        let port = &self.port;

        format!("postgresql://{user}:{password}@{host}:{port}")
    }
}

/// Reads and deserializes configuration.yaml.
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = Config::builder()
        .add_source(File::new("configuration.yaml", FileFormat::Yaml))
        .build()?;

    settings.try_deserialize::<Settings>()
}
