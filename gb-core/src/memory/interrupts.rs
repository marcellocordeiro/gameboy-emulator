use bitflags::bitflags;

bitflags! {
    #[derive(Default, Clone, Copy)]
    pub struct InterruptBits: u8 {
        const JOYPAD   = 1 << 4;
        const SERIAL   = 1 << 3;
        const TIMER    = 1 << 2;
        const LCD_STAT = 1 << 1;
        const VBLANK   = 1 << 0;
    }
}

const INTERRUPT_PRIORITY: [InterruptBits; 5] = [
    InterruptBits::VBLANK,
    InterruptBits::LCD_STAT,
    InterruptBits::TIMER,
    InterruptBits::SERIAL,
    InterruptBits::JOYPAD,
];

#[derive(Default)]
pub struct Interrupts {
    pub flags: InterruptBits,  // IF.
    pub enable: InterruptBits, // IE.
}

impl Interrupts {
    pub fn skip_bootrom(&mut self) {
        self.flags = InterruptBits::from_bits_truncate(0xE1);
    }

    pub fn get_queued_irq(&self) -> Option<(InterruptBits, usize)> {
        if self.enable.is_empty() || self.flags.is_empty() {
            return None;
        }

        let intersection = self.enable & self.flags;

        for (index, interrupt) in INTERRUPT_PRIORITY.iter().enumerate() {
            if intersection.contains(*interrupt) {
                return Some((*interrupt, index));
            }
        }

        None
    }

    pub fn take_queued_irq(&mut self) -> Option<u16> {
        let (interrupt, index) = self.get_queued_irq()?;

        self.flags.remove(interrupt);

        let address = (0x40 + 0x08 * index) as u16;

        Some(address)
    }

    pub fn read_flags(&self) -> u8 {
        0xE0 | self.flags.bits()
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
