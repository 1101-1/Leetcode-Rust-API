use tokio::io;

#[derive(thiserror::Error, Debug)]
pub enum Errors {
    #[error("BuildError(Can not collect problems data caused by `{0}`)")]
    BuildError(#[from] serde_json::Error),
    #[error("FetchProblemError(Can not fetch problems data. Request error)")]
    FetchProblemError(#[from] reqwest::Error),
    #[error("ApiError(`{0}`)")]
    ApiError(String),
}

impl std::convert::From<Errors> for io::Error {
    fn from(error: Errors) -> Self {
        io::Error::new(io::ErrorKind::Other, error.to_string())
    }
}
