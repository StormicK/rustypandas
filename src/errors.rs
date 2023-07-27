#[derive(Debug)]
pub enum ProgramError {
    GifAccessError(crate::gif::errors::RepositoryError),
    IoError(std::io::Error),
    VarError(std::env::VarError),
    RestError(reqwest::Error),
    TerminalConfigurationAccessError(crate::configuration::errors::RepositoryError),
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

impl From<crate::gif::errors::RepositoryError> for ProgramError {
    fn from(error: crate::gif::errors::RepositoryError) -> Self {
        ProgramError::GifAccessError(error)
    }
}

impl From<reqwest::Error> for ProgramError {
    fn from(error: reqwest::Error) -> Self {
        ProgramError::RestError(error)
    }
}

impl From<crate::configuration::errors::RepositoryError> for ProgramError {
    fn from(error: crate::configuration::errors::RepositoryError) -> Self {
        ProgramError::TerminalConfigurationAccessError(error)
    }
}