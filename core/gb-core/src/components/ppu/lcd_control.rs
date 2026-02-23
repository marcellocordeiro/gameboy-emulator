use bitflags::bitflags;

use super::{Ppu, lcd_status::StatusMode};
use crate::utils::{color::Color, macros::device_is_cgb};

bitflags!(
    /// FF40 — LCDC: LCD control
    ///
    /// LCDC is the main LCD Control register. Its bits toggle what elements are displayed on the screen, and how.
    ///
    /// | Bit | Name                          | Usage notes              |
    /// | --- | ----------------------------- | ------------------------ |
    /// | 7   | LCD and PPU enable            | 0=Off, 1=On              |
    /// | 6   | Window tile map area          | 0=9800-9BFF, 1=9C00-9FFF |
    /// | 5   | Window enable                 | 0=Off, 1=On              |
    /// | 4   | BG and Window tile data area  | 0=8800-97FF, 1=8000-8FFF |
    /// | 3   | BG tile map area              | 0=9800-9BFF, 1=9C00-9FFF |
    /// | 2   | OBJ size                      | 0=8x8, 1=8x16            |
    /// | 1   | OBJ enable                    | 0=Off, 1=On              |
    /// | 0   | BG and Window enable/priority | 0=Off, 1=On              |
    #[derive(Default, Clone, Copy)]
    pub struct LcdControl: u8 {
        const LCD_ENABLE = 1 << 7;
        const WIN_MAP = 1 << 6;
        const WIN_ENABLE = 1 << 5;
        const BG_WIN_ADDR = 1 << 4;
        const BG_MAP = 1 << 3;
        const OBJ_SIZE = 1 << 2;
        const OBJ_ENABLE = 1 << 1;
        const BG_ENABLE = 1 << 0;
    }
);

impl LcdControl {
    /// LCDC.7
    ///
    /// LCD and PPU enable
    pub fn get_lcd_enable(self) -> bool {
        self.contains(Self::LCD_ENABLE)
    }

    /// LCDC.6
    ///
    /// Window tile map area:
    /// `false => 0x9800-0x9BFF`,
    /// `true => 0x9C00-0x9FFF`
    pub fn get_win_map(self) -> bool {
        self.contains(Self::WIN_MAP)
    }

    /// LCDC.5
    ///
    /// Window enable
    pub fn get_win_enable(self) -> bool {
        self.contains(Self::WIN_ENABLE)
    }

    /// LCDC.4
    ///
    /// BG and Window tile data area:
    /// `false => 0x8800-0x97FF`,
    /// `true => 0x8000-0x8FFF`
    pub fn get_bg_win_addr(self) -> bool {
        self.contains(Self::BG_WIN_ADDR)
    }

    /// LCDC.3
    ///
    /// BG tile map area:
    /// `false => 0x9800-0x9BFF`,
    /// `true => 0x9C00-0x9FFF`
    pub fn get_bg_map(self) -> bool {
        self.contains(Self::BG_MAP)
    }

    /// LCDC.2
    ///
    /// OBJ size:
    /// `false => 8x8`,
    /// `true => 8x16`
    pub fn get_obj_size(self) -> bool {
        self.contains(Self::OBJ_SIZE)
    }

    /// LCDC.1
    ///
    /// OBJ enable
    pub fn get_obj_enable(self) -> bool {
        self.contains(Self::OBJ_ENABLE)
    }

    /// LCDC.0
    ///
    /// BG and Window enable/priority
    ///
    /// Different meanings for DMG and CGB.
    pub fn get_bg_enable(self) -> bool {
        self.contains(Self::BG_ENABLE)
    }
}

impl Ppu {
    #[must_use]
    pub fn read_lcdc(&self) -> u8 {
        self.lcdc.bits()
    }

    pub fn write_lcdc(&mut self, value: u8) {
        let new_lcdc = LcdControl::from_bits_truncate(value);

        let lcd_enable = self.lcdc.get_lcd_enable();
        let new_lcd_enable = new_lcdc.get_lcd_enable();

        // Off -> On
        if !lcd_enable && new_lcd_enable {
            self.mode = StatusMode::OamScan;
            self.mode_remaining_dots = StatusMode::OamScan.dots();
        }

        // On -> Off
        if lcd_enable && !new_lcd_enable {
            self.mode = StatusMode::Hblank;
            self.ly = 0;

            if device_is_cgb!(self) {
                self.internal_screen.pixels.fill(Color::CGB_SYSTEM_DEFAULT);
            } else {
                self.internal_screen.pixels.fill(Color::DMG_SYSTEM_DEFAULT);
            }
        }

        self.lcdc = new_lcdc;
    }
}
