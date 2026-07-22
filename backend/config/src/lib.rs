use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub resources: ResourcesConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ResourcesConfig {
    pub tests: TestResourcesConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TestResourcesConfig {
    pub assets: String,
}

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Parse(serde_yml::Error),
}

impl From<std::io::Error> for ConfigError {
    fn from(e: std::io::Error) -> Self {
        ConfigError::Io(e)
    }
}

impl From<serde_yml::Error> for ConfigError {
    fn from(e: serde_yml::Error) -> Self {
        ConfigError::Parse(e)
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigError::Io(e) => write!(f, "Erreur de lecture du fichier: {e}"),
            ConfigError::Parse(e) => write!(f, "Erreur de parsing YAML: {e}"),
        }
    }
}

impl AppConfig {
    pub fn load<P: AsRef<Path>>(chemin: P) -> Result<Self, ConfigError> {
        let contenu = fs::read_to_string(chemin)?;
        let config: AppConfig = serde_yml::from_str(&contenu)?;
        Ok(config)
    }
}
