#[derive(Debug)]
pub enum RepositoryError {
    DataAccessError(String),
    NotFoundError(String),
}

impl From<std::io::Error> for RepositoryError {
    fn from(error: std::io::Error) -> Self {
        RepositoryError::NotFoundError(error.to_string())
    }
}

impl From<serde_json::Error> for RepositoryError {
    fn from(error: serde_json::Error) -> Self {
        RepositoryError::DataAccessError(error.to_string())
    }
}
