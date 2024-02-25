use crate::{utils::macros::in_cgb_mode, DeviceConfig, DeviceModel, OptionalCgbComponent};

#[derive(Debug, Default)]
pub struct SpeedSwitch {
    key0: u8,

    device_config: DeviceConfig,
}

impl OptionalCgbComponent for SpeedSwitch {
    fn with_device_model(model: DeviceModel) -> Self {
        let device_config = DeviceConfig {
            model,
            ..Default::default()
        };

        Self {
            device_config,
            ..Default::default()
        }
    }

    fn set_cgb_mode(&mut self, value: bool) {
        self.device_config.cgb_mode = value;
    }
}

impl SpeedSwitch {
    pub fn in_double_speed(&self) -> bool {
        (self.key0 & 0b1000_0000) != 0
    }

    pub fn process_speed_switch(&mut self) {
        if !in_cgb_mode!(self) {
            return;
        }

        if self.key0 & 0b1 == 1 {
            self.key0 = !self.key0 & 0b1000_0000;
        }
    }

    pub fn read(&self) -> u8 {
        if !in_cgb_mode!(self) {
            return 0xFF;
        }

        0b0111_1110 | self.key0
    }

    pub fn write(&mut self, value: u8) {
        if !in_cgb_mode!(self) {
            return;
        }

        self.key0 |= value & 0b1;
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

        assert_eq!(speed_switch.key0, 0);
        assert_eq!(speed_switch.read(), 0b0111_1110);
        assert!(!speed_switch.in_double_speed());

        // No changes.
        speed_switch.write(0b1111_1110);
        assert_eq!(speed_switch.key0, 0);
        assert_eq!(speed_switch.read(), 0b0111_1110);
        assert!(!speed_switch.in_double_speed());

        // No changes.
        speed_switch.process_speed_switch();
        assert_eq!(speed_switch.key0, 0);
        assert_eq!(speed_switch.read(), 0b0111_1110);
        assert!(!speed_switch.in_double_speed());

        // Request speed switch (Single -> Double).
        speed_switch.write(0b1111_1111);
        assert_eq!(speed_switch.key0, 0b1);
        assert_eq!(speed_switch.read(), 0b0111_1111);
        assert!(!speed_switch.in_double_speed());

        // Process speed switch (Single -> Double).
        speed_switch.process_speed_switch();
        assert_eq!(speed_switch.key0, 0b1000_0000);
        assert_eq!(speed_switch.read(), 0b1111_1110);
        assert!(speed_switch.in_double_speed());

        // Request speed switch (Double -> Single).
        speed_switch.write(0b1111_1111);
        assert_eq!(speed_switch.key0, 0b1000_0001);
        assert_eq!(speed_switch.read(), 0b1111_1111);
        assert!(speed_switch.in_double_speed());

        // Process speed switch (Double -> Single).
        speed_switch.process_speed_switch();
        assert_eq!(speed_switch.key0, 0b0000_0000);
        assert_eq!(speed_switch.read(), 0b0111_1110);
        assert!(!speed_switch.in_double_speed());
    }
}
