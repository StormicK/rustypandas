use crate::models::configuration::ConfigurationModel;
use crate::models::configuration::PresetConfigurator;
use crate::models::errors::ConfigurationModelError;

pub enum ControllerError {
    RefreshPandaError,
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

    pub async fn refresh_panda(&self) -> Result<(), ControllerError> {
        self.configuration_model.refresh_panda().await?;
        Ok(())
    }
}