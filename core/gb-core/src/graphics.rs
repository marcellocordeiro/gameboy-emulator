use crate::{
    cartridge::info::Info,
    utils::{
        macros::{device_is_cgb, in_cgb_mode, pure_read_write_methods_u8},
        screen::Screen,
    },
};

use self::{
    color_ram::ColorRam,
    lcd_control::LcdControl,
    lcd_status::{LcdStatus, StatusMode},
    oam::Oam,
    oam_dma::OamDma,
    video_ram::VideoRam,
    vram_dma::VramDma,
};

#[allow(clippy::struct_excessive_bools)]
#[derive(Default)]
pub struct Graphics {
    // Registers
    lcdc: LcdControl,
    stat: LcdStatus,
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,
    wy: u8,
    wx: u8,
    window_internal_counter: u8,

    bcps: u8,          // (CGB) Background color palette specification
    bg_cram: ColorRam, // (CGB) Accessed through background color palette data (BCPD)

    ocps: u8,           // (CGB) Object color palette specification
    obj_cram: ColorRam, // (CGB) Accessed through object color palette data (OCPD)

    opri: bool, // (CGB) Object priority mode

    pub stat_irq: bool,
    pub vblank_irq: bool,

    pub vram: VideoRam,
    pub oam: Oam,

    pub oam_dma: OamDma,
    pub vram_dma: VramDma,

    mode: StatusMode,
    cycles: u32,

    screen: Screen,
    internal_screen: Screen,

    cgb_mode: bool,
}

impl Graphics {
    pub fn set_cgb_mode(&mut self, value: bool) {
        self.cgb_mode = value;
        self.vram.set_cgb_mode(value);
    }

    pub fn skip_bootrom(&mut self) {
        self.lcdc = LcdControl::from_bits_truncate(0x91);
        self.bgp = 0xFC;
    }

    pub fn handle_post_bootrom_setup(&mut self, info: &Info) {
        if cfg!(feature = "cgb") {
            if !info.cgb_flag.has_cgb_support() {
                self.opri = true;

                for palette_number in 0..8 {
                    self.bg_cram
                        .set_palette(palette_number, [0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF]);
                    self.obj_cram
                        .set_palette(palette_number, [0x0000, 0x0000, 0x0000, 0x0000]);
                }

                let palettes = info.dmg_compatibility_palettes();

                self.bg_cram.set_palette(0, palettes.bg0);
                self.obj_cram.set_palette(0, palettes.obj0);
                self.obj_cram.set_palette(1, palettes.obj1);
            }
        }
    }

    pub fn screen(&self) -> &Screen {
        &self.screen
    }

    pub fn in_hblank(&self) -> bool {
        self.mode == StatusMode::Hblank
    }

    pub fn read_bcps(&self) -> u8 {
        // TODO: lock after bootrom is finished?
        if !device_is_cgb!() {
            return 0xFF;
        }

        0b0100_0000 | self.bcps
    }

    pub fn write_bcps(&mut self, value: u8) {
        // TODO: lock after bootrom is finished?
        if !device_is_cgb!() {
            return;
        }

        self.bcps = value & 0b1011_1111;
    }

    pub fn read_bcpd(&self) -> u8 {
        // TODO: lock after bootrom is finished?
        if !device_is_cgb!() {
            return 0xFF;
        }

        if self.lcdc.get_lcd_enable() && self.mode == StatusMode::Drawing {
            return 0xFF;
        }

        let address = self.bcps & 0b0011_1111;

        self.bg_cram.read(address)
    }

    pub fn write_bcpd(&mut self, value: u8) {
        // TODO: lock after bootrom is finished?

        if !device_is_cgb!() {
            return;
        }

        let increment_address = self.bcps & 0b1000_0000 != 0;
        let address = self.bcps & 0b0011_1111;

        if !(self.lcdc.get_lcd_enable() && self.mode == StatusMode::Drawing) {
            self.bg_cram.write(address, value);
        }

        if increment_address {
            self.bcps &= 0b1000_0000;
            self.bcps |= (address + 1) & 0b0011_1111;
        }
    }

    pub fn read_ocps(&self) -> u8 {
        // TODO: lock after bootrom is finished?
        if !device_is_cgb!() {
            return 0xFF;
        }

        0b0100_0000 | self.ocps
    }

    pub fn write_ocps(&mut self, value: u8) {
        // TODO: lock after bootrom is finished?
        if !device_is_cgb!() {
            return;
        }

        self.ocps = value & 0b1011_1111;
    }

    pub fn read_ocpd(&self) -> u8 {
        // TODO: lock after bootrom is finished?
        if !device_is_cgb!() {
            return 0xFF;
        }

        if self.lcdc.get_lcd_enable() && self.mode == StatusMode::Drawing {
            return 0xFF;
        }

        let address = self.ocps & 0b0011_1111;

        self.obj_cram.read(address)
    }

    pub fn write_ocpd(&mut self, value: u8) {
        // TODO: lock after bootrom is finished?
        if !device_is_cgb!() {
            return;
        }

        let increment_address = self.ocps & 0b1000_0000 != 0;
        let address = self.ocps & 0b0011_1111;

        if !(self.lcdc.get_lcd_enable() && self.mode == StatusMode::Drawing) {
            self.obj_cram.write(address, value);
        }

        if increment_address {
            self.ocps &= 0b1000_0000;
            self.ocps |= (address + 1) & 0b0011_1111;
        }
    }

    pub fn read_opri(&self) -> u8 {
        // TODO: lock after bootrom is finished?
        if !in_cgb_mode!(self) {
            return 0xFF;
        }

        self.opri as u8
    }

    pub fn write_opri(&mut self, value: u8) {
        // TODO: lock after bootrom is finished?
        if !device_is_cgb!() {
            return;
        }

        self.opri = (value & 0b1) != 0;
    }

    pure_read_write_methods_u8! {
        scy,
        scx,
        ly,
        lyc,
        bgp,
        obp0,
        obp1,
        wy,
        wx
    }

    pub fn tick(&mut self) {
        if !self.lcdc.get_lcd_enable() {
            return;
        }

        self.cycles += 1;

        // Quirk.
        // One cycle before the mode switch (Drawing -> Hblank).
        if self.cycles == 251 && self.mode == StatusMode::Drawing {
            if self.stat.get_hblank_irq() {
                self.stat_irq = true;
            }
        }

        match self.mode {
            StatusMode::Hblank => {
                if self.cycles != 456 {
                    return;
                }

                if self.lcdc.get_win_enable()
                    && self.wx < 166
                    && self.wy < 143
                    && self.wy <= self.ly
                {
                    self.window_internal_counter = self.window_internal_counter.wrapping_add(1);
                }

                self.ly += 1;

                if self.ly == 144 {
                    self.switch_mode(StatusMode::Vblank);
                } else {
                    self.switch_mode(StatusMode::OamScan);
                }

                self.cycles = 0;

                self.check_irq();
            }

            StatusMode::Vblank => {
                if self.cycles != 456 {
                    return;
                }

                self.ly += 1;

                if self.ly == 154 {
                    self.ly = 0;
                    self.window_internal_counter = 0;
                    self.switch_mode(StatusMode::OamScan);
                }

                self.cycles = 0;

                self.check_irq();
            }

            StatusMode::OamScan => {
                if self.cycles != 80 {
                    return;
                }

                self.switch_mode(StatusMode::Drawing);
            }

            StatusMode::Drawing => {
                if self.cycles != 252 {
                    return;
                }

                self.draw_line();
                self.switch_mode(StatusMode::Hblank);

                if in_cgb_mode!(self) {
                    self.vram_dma.resume_hdma();
                }
            }
        };
    }

    fn check_irq(&mut self) {
        if self.ly == self.lyc {
            self.stat.insert(LcdStatus::LY_COMPARE);

            if self.stat.get_compare_irq() {
                self.stat_irq = true;
            }
        } else {
            self.stat.remove(LcdStatus::LY_COMPARE);
        }
    }

    fn switch_mode(&mut self, value: StatusMode) {
        self.mode = value;

        match self.mode {
            StatusMode::Hblank => {
                // Handled elsewhere due to different timings.
            }

            StatusMode::Vblank => {
                std::mem::swap(&mut self.screen, &mut self.internal_screen);
                self.vblank_irq = true;

                if self.stat.get_vblank_irq() || self.stat.get_oam_irq() {
                    self.stat_irq = true;
                }
            }

            StatusMode::OamScan => {
                if self.stat.get_oam_irq() {
                    self.stat_irq = true;
                }
            }

            StatusMode::Drawing => {}
        }
    }
}

mod color_ram;
mod debug_getters;
mod draw_line_cgb;
mod draw_line_dmg;
mod lcd_control;
mod lcd_status;
mod oam;
mod oam_dma;
mod sprite;
mod video_ram;
mod vram_dma;
