// TODO: callbacks and stuff.

use std::sync::mpsc;

use bitflags::{Flags, bitflags};

use crate::{constants::DeviceModel, utils::macros::in_cgb_mode};

#[derive(Debug, Default)]
pub struct Serial {
    sb: u8,      // Serial transfer data (R/W)
    sc: Control, // Serial transfer control (R/W)

    pub irq: bool,

    cgb_mode: bool,
    device_model: DeviceModel,

    sender: Option<mpsc::Sender<u8>>,
}

bitflags! {
    #[derive(Debug, Default)]
    struct Control: u8 {
        const TRANSFER_ENABLE = 1 << 7;
        const CLOCK_SPEED = 1 << 1;
        const CLOCK_SELECT = 1 << 0;
    }
}

impl Serial {
    #[must_use]
    pub fn with_device_model(device_model: DeviceModel) -> Self {
        Self {
            cgb_mode: device_model.is_cgb(),
            device_model,
            ..Default::default()
        }
    }

    pub fn set_cgb_mode(&mut self, value: bool) {
        self.cgb_mode = value;
    }

    #[must_use]
    pub fn read_sb(&self) -> u8 {
        self.sb
    }

    pub fn write_sb(&mut self, value: u8) {
        self.sb = value;
    }

    #[must_use]
    pub fn read_sc(&self) -> u8 {
        let unused_mask = if in_cgb_mode!(self) {
            0b0111_1100
        } else {
            0b0111_1110
        };
        self.sc.bits() | unused_mask
    }

    pub fn write_sc(&mut self, value: u8) {
        let mask = if in_cgb_mode!(self) {
            0b1000_0011
        } else {
            0b1000_0001
        };
        self.sc = Control::from_bits_truncate(value & mask);

        // (SC & 0x81) == 0x81
        if self
            .sc
            .contains(Control::TRANSFER_ENABLE | Control::CLOCK_SELECT)
        {
            if let Some(sender) = self.sender.as_mut() {
                sender.send(self.sb).unwrap();
            } else {
                // Print serial output for tests.
                // print!("{}", self.sb as char);
            }

            self.irq = true;
            self.sc.clear();
        }
    }

    pub fn add_sender(&mut self, sender: mpsc::Sender<u8>) {
        self.sender = Some(sender);
    }
}
