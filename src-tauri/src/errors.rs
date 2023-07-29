#[derive(Debug)]
pub enum ProgramError {
    GifAccessError(crate::repositories::gif::errors::RepositoryError),
    IoError(std::io::Error),
    VarError(std::env::VarError),
    RestError(reqwest::Error),
    TerminalConfigurationAccessError(crate::repositories::configuration::errors::RepositoryError),
    ModelError(crate::models::errors::ConfigurationModelError),
}

impl From<std::io::Error> for ProgramError {
    fn from(error: std::io::Error) -> Self {
        ProgramError::IoError(error)
    }
}

impl From<std::env::VarError> for ProgramError {
    fn from(error: std::env::VarError) -> Self {
        ProgramError::VarError(error)
    }
}

impl From<crate::repositories::gif::errors::RepositoryError> for ProgramError {
    fn from(error: crate::repositories::gif::errors::RepositoryError) -> Self {
        ProgramError::GifAccessError(error)
    }
}

impl From<reqwest::Error> for ProgramError {
    fn from(error: reqwest::Error) -> Self {
        ProgramError::RestError(error)
    }
}

impl From<crate::repositories::configuration::errors::RepositoryError> for ProgramError {
    fn from(error: crate::repositories::configuration::errors::RepositoryError) -> Self {
        ProgramError::TerminalConfigurationAccessError(error)
    }
}

impl From<crate::models::errors::ConfigurationModelError> for ProgramError {
    fn from(error: crate::models::errors::ConfigurationModelError) -> Self {
        ProgramError::ModelError(error)
    }
}