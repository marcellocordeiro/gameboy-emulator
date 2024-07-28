use thiserror::Error;

use crate::components::{cartridge::error::CartridgeError, memory::bootrom::BootromError};

#[derive(Debug, Error)]
pub enum GameBoyError {
    #[error("Failed to load the bootrom: {0}.")]
    BootromError(#[from] BootromError),

    #[error("Failed to load the cartridge: {0}.")]
    CartridgeError(#[from] CartridgeError),
}
