#[derive(Debug, Default)]
pub struct VramDma {}

impl VramDma {
    pub fn read(&self, address: u16) -> u8 {
        match address {
            // HDMA1 (source high)
            0xFF51 => 0xFF,

            // HDMA2 (source low)
            0xFF52 => 0xFF,

            // HDMA3 (destination high)
            0xFF53 => 0xFF,

            // HDMA4 (destination low)
            0xFF54 => 0xFF,

            // HDMA5 (length/mode/start)
            0xFF55 => 0xFF,

            _ => unreachable!("[vram_dma.rs] Read out of bounds: {address:#06x}"),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            // HDMA1 (source high)
            0xFF51 => (),

            // HDMA2 (source low)
            0xFF52 => (),

            // HDMA3 (destination high)
            0xFF53 => (),

            // HDMA4 (destination low)
            0xFF54 => (),

            // HDMA5 (length/mode/start)
            0xFF55 => (),

            _ => unreachable!("[vram_dma.rs] Write out of bounds: ({address:#06x}) = {value:#04x}"),
        }
    }
}
