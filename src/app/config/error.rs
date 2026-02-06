use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing env var `{0}`")]
    MissingVar(String),

    #[error("invalid value: {0}")]
    InvalidValue(#[from] std::num::ParseIntError),

    #[error("env error: {0}")]
    Env(#[from] std::env::VarError),
}
