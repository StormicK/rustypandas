use async_trait::async_trait;
use crate::repositories::configuration::dto::TerminalConfig;
use crate::repositories::configuration::errors::RepositoryError;

#[async_trait]
pub trait TerminalConfigurationRepository {
    async fn get_configuration(&self) -> Result<TerminalConfig, RepositoryError>;
    async fn update_configuration(&self, configuration: TerminalConfig) -> Result<(), RepositoryError>;
}

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
        let file = tokio::fs::read(&self.path).await?;
        
        let configuration = serde_json::from_slice(&file)?;
        Ok(configuration)
    }

    async fn update_configuration(&self, configuration: TerminalConfig) -> Result<(), RepositoryError> {
        let file = serde_json::to_vec(&configuration)?;
        tokio::fs::write(&self.path, file).await?;
        Ok(())
    }
}