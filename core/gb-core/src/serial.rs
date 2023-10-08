// TODO: callbacks and stuff.

use std::sync::mpsc;

#[derive(Default)]
pub struct Serial {
    sb: u8, // Serial transfer data (R/W).
    sc: u8, // Serial transfer control (R/W).

    pub irq: bool,

    sender: Option<mpsc::Sender<u8>>,
}

impl Serial {
    pub fn add_sender(&mut self, sender: mpsc::Sender<u8>) {
        self.sender = Some(sender);
    }

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
                    if let Some(sender) = self.sender.as_mut() {
                        sender.send(self.sb).unwrap();
                    } else {
                        // Print serial output for tests.
                        // print!("{}", self.sb as char);
                    }
                }
            }

            _ => unreachable!("[serial.rs] Invalid write: ({address:#06x}) = {value:#04x}"),
        }
    }
}
