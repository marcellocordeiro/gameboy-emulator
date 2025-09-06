use crate::{constants::DeviceModel, utils::macros::device_is_cgb};

pub struct Key0 {
    pub cgb_mode: bool,
    pub device_model: DeviceModel,

    locked_bootrom: bool,
}

impl Key0 {
    pub fn with_device_model(device_model: DeviceModel) -> Self {
        Self {
            cgb_mode: device_model.is_cgb(),
            device_model,
            locked_bootrom: false,
        }
    }

    pub fn set_cgb_mode(&mut self, value: bool) {
        log::info!("CGB mode: ({})", if value { "enabled" } else { "disabled" });

        self.cgb_mode = value;
    }

    pub fn handle_locked_bootrom(&mut self) {
        self.locked_bootrom = true;
    }

    pub fn read(&self) -> u8 {
        if !device_is_cgb!(self) || self.locked_bootrom {
            return 0xFF;
        }

        let dmg_compatibility_mode_bit = (!self.cgb_mode as u8) << 2;

        dmg_compatibility_mode_bit | 0b1111_1011
    }

    pub fn write(&mut self, value: u8) {
        if !device_is_cgb!(self) || self.locked_bootrom {
            return;
        }

        // Bit 3 means:
        // 0: CGB mode
        // 1: DMG compatibility mode
        self.cgb_mode = (value & 0b0000_0100) == 0;
    }
}
