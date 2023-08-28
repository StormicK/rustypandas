use async_trait::async_trait;

use crate::{
    models::errors::ModelError,
    repositories::configuration::repository::{
        JsonConfigurationRepository, TerminalConfigurationRepository,
    },
};

#[async_trait]
pub trait ColorSchemesModelTrait {
    async fn get_color_schemes(&self) -> Result<Vec<String>, ModelError>;
}

#[derive(Debug)]
pub struct ColorSchemesModel {
    terminal_config_repository: JsonConfigurationRepository,
}

impl ColorSchemesModel {
    pub fn new(terminal_config_repository: JsonConfigurationRepository) -> Self {
        Self {
            terminal_config_repository,
        }
    }
}

#[async_trait]
impl ColorSchemesModelTrait for ColorSchemesModel {
    async fn get_color_schemes(&self) -> Result<Vec<String>, ModelError> {
        let terminal_config = self.terminal_config_repository.get_configuration().await?;

        let color_schemes: Vec<String> = terminal_config
            .schemes
            .iter()
            .map(|s| s.name.to_string())
            .collect();

        Ok(color_schemes)
    }
}

#[cfg(test)]
mod test;