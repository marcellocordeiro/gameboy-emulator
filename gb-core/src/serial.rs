// TODO: callbacks and stuff.

#[derive(Default)]
pub struct Serial {
    sb: u8, // Serial transfer data (R/W).
    sc: u8, // Serial transfer control (R/W).

    pub irq: bool,
}

impl Serial {
    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xFF01 => self.sb,
            0xFF02 => self.sc | 0x7E,

            _ => unreachable!("[serial.rs] Invalid read: {:#06x}", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF01 => self.sb = value,
            0xFF02 => {
                self.sc = value;

                if self.sc == 0x81 {
                    // Print serial output for tests.
                    print!("{}", self.sb as char);
                }
            }

            _ => unreachable!(
                "[serial.rs] Invalid write: ({:#06x}) = {:#04x}",
                address, value
            ),
        }
    }
}
