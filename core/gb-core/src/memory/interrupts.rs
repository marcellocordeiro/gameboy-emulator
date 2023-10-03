use bitflags::bitflags;

use crate::utils::bits;

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct InterruptBits: u8 {
        const VBLANK = 1 << 0;
        const LCD_STAT = 1 << 1;
        const TIMER = 1 << 2;
        const SERIAL = 1 << 3;
        const JOYPAD = 1 << 4;
    }
}

#[derive(Default)]
pub struct Interrupts {
    pub flags: InterruptBits,  // IF.
    pub enable: InterruptBits, // IE.
}

impl Interrupts {
    pub fn skip_bootrom(&mut self) {
        self.flags = InterruptBits::from_bits_truncate(0xE1);
    }

    pub fn has_queued_irq(&self) -> bool {
        let intersection = self.enable & self.flags;

        !intersection.is_empty()
    }

    pub fn take_queued_irq(&mut self) -> Option<u16> {
        let interrupt = self.get_queued_irq()?;

        let address = match interrupt {
            InterruptBits::VBLANK => 0x0040,
            InterruptBits::LCD_STAT => 0x0048,
            InterruptBits::TIMER => 0x0050,
            InterruptBits::SERIAL => 0x0058,
            InterruptBits::JOYPAD => 0x0060,

            _ => return None,
        };

        self.flags.remove(interrupt);

        Some(address)
    }

    fn get_queued_irq(&self) -> Option<InterruptBits> {
        let intersection = self.enable & self.flags;

        if intersection.is_empty() {
            return None;
        }

        let bits = intersection.bits();
        let result = bits::isolate_rightmost_bit(bits);

        let interrupt = InterruptBits::from_bits_truncate(result);

        Some(interrupt)
    }

    pub fn read_flags(&self) -> u8 {
        0b1110_0000 | self.flags.bits()
    }

    pub fn read_enable(&self) -> u8 {
        self.enable.bits()
    }

    pub fn write_flags(&mut self, value: u8) {
        self.flags = InterruptBits::from_bits_truncate(value);
    }

    pub fn write_enable(&mut self, value: u8) {
        self.enable = InterruptBits::from_bits_truncate(value);
    }

    // IRQ helpers.
    pub fn request_vblank(&mut self) {
        self.flags.insert(InterruptBits::VBLANK);
    }

    pub fn request_lcd_stat(&mut self) {
        self.flags.insert(InterruptBits::LCD_STAT);
    }

    pub fn request_timer(&mut self) {
        self.flags.insert(InterruptBits::TIMER);
    }

    pub fn request_serial(&mut self) {
        self.flags.insert(InterruptBits::SERIAL);
    }

    pub fn request_joypad(&mut self) {
        self.flags.insert(InterruptBits::JOYPAD);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queued_irq() {
        let mut interrupts = Interrupts::default();

        interrupts.request_lcd_stat();
        interrupts.request_serial();

        interrupts.write_enable(0b0_1010);

        assert_eq!(interrupts.read_enable(), 0b0_1010);
        assert_eq!(interrupts.read_flags(), 0b1110_0000 | 0b0_1010);

        // Test LCD_STAT
        assert!(interrupts.has_queued_irq());
        assert_eq!(interrupts.get_queued_irq(), Some(InterruptBits::LCD_STAT));

        let queued_irq = interrupts.take_queued_irq().unwrap();

        assert_eq!(queued_irq, 0x0048);

        assert_eq!(interrupts.read_enable(), 0b0_1010);
        assert_eq!(interrupts.read_flags(), 0b1110_0000 | 0b0_1000);

        // Test serial
        assert!(interrupts.has_queued_irq());
        assert_eq!(interrupts.get_queued_irq(), Some(InterruptBits::SERIAL));

        let queued_irq = interrupts.take_queued_irq().unwrap();

        assert_eq!(queued_irq, 0x0058);

        assert_eq!(interrupts.read_enable(), 0b0_1010);
        assert_eq!(interrupts.read_flags(), 0b1110_0000);

        // Test none
        assert!(!interrupts.has_queued_irq());
        assert_eq!(interrupts.get_queued_irq(), None);
        assert_eq!(interrupts.take_queued_irq(), None);
    }
}
