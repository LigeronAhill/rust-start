use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("error setting default logger")]
    LoggerError(#[from] tracing::subscriber::SetGlobalDefaultError),
    #[error("error getting configuration from environment variables or file")]
    ConfigError(#[from] config::ConfigError),
    #[error("wrong input parameters")]
    WrongInputParameters,
}

pub type AppResult<T> = Result<T, AppError>;
