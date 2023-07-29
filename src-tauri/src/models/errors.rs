use crate::repositories::{
    configuration::errors::RepositoryError as TerminalConfigurationRepositoryError,
    gif::errors::RepositoryError as GifRepositoryError,
};

#[derive(Debug)]
pub enum ConfigurationModelError {
    ConfigurationFailedError(),
}

impl From<TerminalConfigurationRepositoryError> for ConfigurationModelError {
    fn from(_: TerminalConfigurationRepositoryError) -> Self {
        ConfigurationModelError::ConfigurationFailedError()
    }
}

impl From<GifRepositoryError> for ConfigurationModelError {
    fn from(_: GifRepositoryError) -> Self {
        ConfigurationModelError::ConfigurationFailedError()
    }
}
