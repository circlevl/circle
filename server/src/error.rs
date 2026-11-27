use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomError {
    #[error("connection error {0}")]
    IO(#[from] std::io::Error),

    #[error("wincode read error")]
    WincodeReadError(#[from] wincode::ReadError),

    #[error("wincode write error")]
    WincodeWriteError(#[from] wincode::WriteError),
}
