use crate::{DeviceModel, utils::macros::in_cgb_mode};

#[derive(Debug, Default)]
pub struct SpeedSwitch {
    double_speed: bool,
    armed: bool,

    cgb_mode: bool,
    device_model: DeviceModel,
}

impl SpeedSwitch {
    // 0xFF4D: KEY1 - Prepare speed switch

    pub fn with_device_model(device_model: DeviceModel) -> Self {
        Self {
            device_model,
            ..Default::default()
        }
    }

    pub fn set_cgb_mode(&mut self, value: bool) {
        self.cgb_mode = value;
    }

    pub fn double_speed(&self) -> bool {
        self.double_speed
    }

    pub fn armed(&self) -> bool {
        self.armed
    }

    pub fn process(&mut self) {
        if !in_cgb_mode!(self) {
            return;
        }

        if self.armed {
            self.double_speed = !self.double_speed;
            self.armed = false;
            log::info!(
                "Speed switch processed (double speed: {})",
                self.double_speed
            );
        }
    }

    pub fn read(&self) -> u8 {
        if !in_cgb_mode!(self) {
            return 0xFF;
        }

        ((self.double_speed as u8) << 7) | (self.armed as u8) | 0b0111_1110
    }

    pub fn write(&mut self, value: u8) {
        if !in_cgb_mode!(self) {
            return;
        }

        self.armed = (value & 0b1) != 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_sanity_dmg() {
        let mut speed_switch = SpeedSwitch::with_device_model(DeviceModel::Dmg);

        assert_eq!(speed_switch.read(), 0xFF);
        speed_switch.write(0xFF);
        assert_eq!(speed_switch.read(), 0xFF);
    }

    #[test]
    fn test_my_sanity_cgb() {
        let mut speed_switch = SpeedSwitch::with_device_model(DeviceModel::Cgb);
        speed_switch.set_cgb_mode(true);

        assert_eq!(speed_switch.read(), 0b0111_1110);

        speed_switch.write(0xFF);
        assert_eq!(speed_switch.read(), 0b0111_1111);
    }

    #[test]
    fn test_switch() {
        let mut speed_switch = SpeedSwitch::with_device_model(DeviceModel::Cgb);
        speed_switch.set_cgb_mode(true);

        assert!(!speed_switch.double_speed);
        assert!(!speed_switch.armed);
        assert_eq!(speed_switch.read(), 0b0111_1110);

        // No changes.
        speed_switch.write(0b1111_1110);
        assert!(!speed_switch.double_speed);
        assert!(!speed_switch.armed);
        assert_eq!(speed_switch.read(), 0b0111_1110);

        // No changes.
        speed_switch.process();
        assert!(!speed_switch.double_speed);
        assert!(!speed_switch.armed);
        assert_eq!(speed_switch.read(), 0b0111_1110);

        // Request speed switch (Single -> Double).
        speed_switch.write(0b1111_1111);
        assert!(!speed_switch.double_speed);
        assert!(speed_switch.armed);
        assert_eq!(speed_switch.read(), 0b0111_1111);

        // Process speed switch (Single -> Double).
        speed_switch.process();
        assert!(speed_switch.double_speed);
        assert!(!speed_switch.armed);
        assert_eq!(speed_switch.read(), 0b1111_1110);

        // Request speed switch (Double -> Single).
        speed_switch.write(0b1111_1111);
        assert!(speed_switch.double_speed);
        assert!(speed_switch.armed);
        assert_eq!(speed_switch.read(), 0b1111_1111);

        // Process speed switch (Double -> Single).
        speed_switch.process();
        assert!(!speed_switch.double_speed);
        assert!(!speed_switch.armed);
        assert_eq!(speed_switch.read(), 0b0111_1110);
    }
}
