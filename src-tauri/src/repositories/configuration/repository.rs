use async_trait::async_trait;
use crate::repositories::configuration::dto_rework::testdto::WindowsTerminalConfigurationClass as TerminalConfig;
use crate::repositories::configuration::errors::RepositoryError;

#[async_trait]
pub trait TerminalConfigurationRepository {
    async fn get_configuration(&self) -> Result<TerminalConfig, RepositoryError>;
    async fn update_configuration(&self, configuration: TerminalConfig) -> Result<(), RepositoryError>;
}

#[derive(Debug)]
pub struct JsonConfigurationRepository {
    path: String,
}

impl JsonConfigurationRepository {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

#[async_trait]
impl TerminalConfigurationRepository for JsonConfigurationRepository {
    async fn get_configuration(&self) -> Result<TerminalConfig, RepositoryError> {
        let file = tokio::fs::read(&self.path).await.unwrap();
        
        let configuration = serde_json::from_slice(&file).unwrap();
        Ok(configuration)
    }

    async fn update_configuration(&self, configuration: TerminalConfig) -> Result<(), RepositoryError> {
        let file = serde_json::to_string_pretty(&configuration).unwrap();
        tokio::fs::write(&self.path, file).await.unwrap();
        Ok(())
    }
}