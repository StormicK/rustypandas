use crate::repositories::{
    configuration::errors::RepositoryError as TerminalConfigurationRepositoryError,
    gif::errors::RepositoryError as GifRepositoryError,
};

#[derive(Debug, PartialEq)]
pub enum ModelError {
    ConfigurationFailedError(),
}

impl From<TerminalConfigurationRepositoryError> for ModelError {
    fn from(_: TerminalConfigurationRepositoryError) -> Self {
        ModelError::ConfigurationFailedError()
    }
}

impl From<GifRepositoryError> for ModelError {
    fn from(_: GifRepositoryError) -> Self {
        ModelError::ConfigurationFailedError()
    }
}
