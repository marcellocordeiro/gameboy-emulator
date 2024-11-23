use crate::{
    DeviceModel,
    utils::macros::{device_is_cgb, in_cgb_mode},
};

#[derive(Debug, Default)]
pub struct UndocumentedRegisters {
    reg_0xff72: u8,
    reg_0xff73: u8,
    reg_0xff74: u8,
    reg_0xff75: u8,

    cgb_mode: bool,
    device_model: DeviceModel,
}

impl UndocumentedRegisters {
    const REG_FF75_MASK: u8 = 0b0111_0000;

    pub fn with_device_model(device_model: DeviceModel) -> Self {
        Self {
            device_model,
            ..Default::default()
        }
    }

    pub fn set_cgb_mode(&mut self, value: bool) {
        self.cgb_mode = value;
    }

    pub fn read_0xff72(&self) -> u8 {
        if !device_is_cgb!(self) {
            return 0xFF;
        }

        self.reg_0xff72
    }

    pub fn read_0xff73(&self) -> u8 {
        if !device_is_cgb!(self) {
            return 0xFF;
        }

        self.reg_0xff73
    }

    pub fn read_0xff74(&self) -> u8 {
        if !in_cgb_mode!(self) {
            return 0xFF;
        }

        self.reg_0xff74
    }

    pub fn read_0xff75(&self) -> u8 {
        if !device_is_cgb!(self) {
            return 0xFF;
        }

        !Self::REG_FF75_MASK | self.reg_0xff75
    }

    pub fn write_0xff72(&mut self, value: u8) {
        if !device_is_cgb!(self) {
            return;
        }

        self.reg_0xff72 = value;
    }

    pub fn write_0xff73(&mut self, value: u8) {
        if !device_is_cgb!(self) {
            return;
        }

        self.reg_0xff73 = value;
    }

    pub fn write_0xff74(&mut self, value: u8) {
        if !in_cgb_mode!(self) {
            return;
        }

        self.reg_0xff74 = value;
    }

    pub fn write_0xff75(&mut self, value: u8) {
        if !device_is_cgb!(self) {
            return;
        }

        self.reg_0xff75 = value & Self::REG_FF75_MASK;
    }
}
