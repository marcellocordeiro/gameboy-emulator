use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Timed out.")]
    Timeout,
    #[error("The test reported an error {0}.")]
    MemoryOutputFailure(String),
    #[error("The test reported an error {0}.")]
    SerialOutputFailure(String),
    #[error("Assertion failed. The Fibonacci valitation failed.")]
    FibonacciValidationFailure,
    #[error("Assertion failed. The snapshot does not match the expected one.")]
    SnapshotMismatch,
    #[error("Cartridge error: {0:?}")]
    CartridgeError(#[from] gb_core::cartridge::error::Error),
    #[error("Internal image error: {0:?}")]
    ImageError(#[from] image::ImageError),
}
