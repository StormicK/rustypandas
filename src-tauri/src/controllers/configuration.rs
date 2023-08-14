use crate::models::configuration::ConfigurationModel;
use crate::models::configuration::ConfigurationModelTrait;
use crate::models::errors::ConfigurationModelError;
use async_trait::async_trait;

pub enum ControllerError {
    RefreshPandaError,
}

#[async_trait]
pub trait ConfigurationControllerTrait {
    async fn update_gif(&self, search_query: &str) -> Result<(), ControllerError>;
    async fn update_color_scheme(&self, color_scheme: &str) -> Result<(), ControllerError>;
    async fn get_color_schemes(&self) -> Result<Vec<String>, ControllerError>;
}

impl From<ConfigurationModelError> for ControllerError {
    fn from(_: ConfigurationModelError) -> Self {
        Self::RefreshPandaError
    }
}

pub struct ConfigurationController {
    configuration_model: ConfigurationModel,
}

impl ConfigurationController {
    pub fn new(configuration_model: ConfigurationModel) -> Self {
        Self {
            configuration_model,
        }
    }
}

#[async_trait]
impl ConfigurationControllerTrait for ConfigurationController {
    async fn update_gif(&self, search_query: &str) -> Result<(), ControllerError> {
        self.configuration_model.update_gif(search_query).await?;
        Ok(())
    }
    async fn update_color_scheme(&self, color_scheme: &str) -> Result<(), ControllerError> {
        self.configuration_model.update_color_scheme(color_scheme).await?;
        Ok(())
    }
    async fn get_color_schemes(&self) -> Result<Vec<String>, ControllerError> {
        let color_schemes = self.configuration_model.get_color_schemes().await?;
        Ok(color_schemes)
    }
}