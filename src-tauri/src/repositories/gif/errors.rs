#[derive(Debug)]
pub enum RepositoryError {
    DataAccessError(String),
    NotFoundError(String),
}

impl From<url::ParseError> for RepositoryError {
    fn from(error: url::ParseError) -> Self {
        RepositoryError::DataAccessError(error.to_string())
    }
}

impl From<reqwest::Error> for RepositoryError {
    fn from(error: reqwest::Error) -> Self {
        RepositoryError::DataAccessError(error.to_string())
    }
}

impl From<std::io::Error> for RepositoryError {
    fn from(error: std::io::Error) -> Self {
        RepositoryError::DataAccessError(error.to_string())
    }
}

impl From<serde_json::Error> for RepositoryError {
    fn from(error: serde_json::Error) -> Self {
        RepositoryError::DataAccessError(error.to_string())
    }
}

impl From<std::env::VarError> for RepositoryError {
    fn from(error: std::env::VarError) -> Self {
        RepositoryError::DataAccessError(error.to_string())
    }
}
