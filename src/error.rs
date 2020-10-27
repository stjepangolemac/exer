use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExerError {
    #[error("Could not initialize templating")]
    Templating(#[from] tera::Error),
    #[error("Could not get working directory")]
    WorkingDirectory(#[from] std::io::Error),
}

pub type ExerResult<T> = Result<T, ExerError>;
