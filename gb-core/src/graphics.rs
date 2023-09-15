use crate::constants::{Frame, Framebuffer, SCREEN_HEIGHT, SCREEN_WIDTH};

use self::{
    color::Color,
    lcd_control::LcdControl,
    lcd_status::{LcdStatus, StatusMode},
    oam::Oam,
    oam_dma::OamDma,
    sprite::SpriteFlags,
    video_ram::VideoRam,
    vram_dma::VramDma,
};

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

    bcps: u8,                 // (CGB) Background color palette specification
    bg_palette_ram: [u8; 64], // (CGB) Accessed through background color palette data (BCPD)

    ocps: u8,                  // (CGB) Object color palette specification
    obj_palette_ram: [u8; 64], // (CGB) Accessed through object color palette data (OCPD)

    pub stat_irq: bool,
    pub vblank_irq: bool,

    pub vram: VideoRam,
    pub oam: Oam,

    pub oam_dma: OamDma,
    pub vram_dma: VramDma,

    mode: StatusMode,
    cycles: u32,

    framebuffer: Framebuffer,

    cgb_mode: bool,
}

impl Default for Graphics {
    fn default() -> Self {
        Self {
            lcdc: LcdControl::default(),
            stat: LcdStatus::default(),
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            bgp: 0,
            obp0: 0xFF,
            obp1: 0xFF,
            wy: 0,
            wx: 0,
            window_internal_counter: 0,

            bcps: 0,
            bg_palette_ram: [0; 64],

            ocps: 0,
            obj_palette_ram: [0; 64],

            stat_irq: false,
            vblank_irq: false,

            vram: VideoRam::default(),
            oam: Oam::default(),

            oam_dma: OamDma::default(),
            vram_dma: VramDma::default(),

            mode: StatusMode::OamScan,
            cycles: 0,

            framebuffer: [Color::white(); SCREEN_WIDTH * SCREEN_HEIGHT],

            cgb_mode: false,
        }
    }
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

    pub fn draw_into_frame(&self, frame: &mut Frame) {
        for (i, pixel) in self.framebuffer.iter().enumerate() {
            frame[i * 4] = pixel.red;
            frame[(i * 4) + 1] = pixel.green;
            frame[(i * 4) + 2] = pixel.blue;
            frame[(i * 4) + 3] = pixel.alpha;
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xFF40 => self.read_lcdc(),
            0xFF41 => self.read_stat(),
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF46 => self.oam_dma.read(),
            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,

            0xFF68 => self.read_bcps(),
            0xFF69 => self.read_bcpd(),
            0xFF6A => self.read_ocps(),
            0xFF6B => self.read_ocpd(),

            _ => {
                unreachable!("[video.rs] Read out of bounds: {address:#06x}");
            }
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF40 => self.write_lcdc(value),
            0xFF41 => self.write_stat(value),
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF44 => println!("[video.rs] LY is read-only."),
            0xFF45 => self.lyc = value,
            0xFF46 => self.oam_dma.write(value),
            0xFF47 => self.bgp = value,
            0xFF48 => self.obp0 = value,
            0xFF49 => self.obp1 = value,
            0xFF4A => self.wy = value,
            0xFF4B => self.wx = value,

            0xFF68 => self.write_bcps(value),
            0xFF69 => self.write_bcpd(value),
            0xFF6A => self.write_ocps(value),
            0xFF6B => self.write_ocpd(value),

            _ => {
                unreachable!("[video.rs] Write out of bounds: ({address:#06x}) = {value:#04x}");
            }
        }
    }

    fn read_bcps(&self) -> u8 {
        if !(cfg!(feature = "cgb") && self.cgb_mode) {
            return 0xFF;
        }

        0b0100_0000 | self.bcps
    }

    fn write_bcps(&mut self, value: u8) {
        if !(cfg!(feature = "cgb") && self.cgb_mode) {
            return;
        }

        self.bcps = value & 0b1011_1111;
    }

    fn read_bcpd(&self) -> u8 {
        if !(cfg!(feature = "cgb") && self.cgb_mode) {
            return 0xFF;
        }

        if self.lcdc.get_lcd_enable() && self.mode == StatusMode::Drawing {
            return 0xFF;
        }

        let address = self.bcps & 0b0011_1111;

        self.bg_palette_ram[address as usize]
    }

    fn write_bcpd(&mut self, value: u8) {
        if !(cfg!(feature = "cgb") && self.cgb_mode) {
            return;
        }

        let increment_address = self.bcps & 0b1000_0000 != 0;
        let address = self.bcps & 0b0011_1111;

        if !(self.lcdc.get_lcd_enable() && self.mode == StatusMode::Drawing) {
            self.bg_palette_ram[address as usize] = value;
        }

        if increment_address {
            self.bcps &= 0b1000_0000;
            self.bcps |= (address + 1) & 0b0011_1111;
        }
    }

    fn read_ocps(&self) -> u8 {
        if !(cfg!(feature = "cgb") && self.cgb_mode) {
            return 0xFF;
        }

        0b0100_0000 | self.ocps
    }

    fn write_ocps(&mut self, value: u8) {
        if !(cfg!(feature = "cgb") && self.cgb_mode) {
            return;
        }

        self.ocps = value & 0b1011_1111;
    }

    fn read_ocpd(&self) -> u8 {
        if !(cfg!(feature = "cgb") && self.cgb_mode) {
            return 0xFF;
        }

        if self.lcdc.get_lcd_enable() && self.mode == StatusMode::Drawing {
            return 0xFF;
        }

        let address = self.ocps & 0b0011_1111;

        self.obj_palette_ram[address as usize]
    }

    fn write_ocpd(&mut self, value: u8) {
        if !(cfg!(feature = "cgb") && self.cgb_mode) {
            return;
        }

        let increment_address = self.ocps & 0b1000_0000 != 0;
        let address = self.ocps & 0b0011_1111;

        if !(self.lcdc.get_lcd_enable() && self.mode == StatusMode::Drawing) {
            self.obj_palette_ram[address as usize] = value;
        }

        if increment_address {
            self.ocps &= 0b1000_0000;
            self.ocps |= (address + 1) & 0b0011_1111;
        }
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
                    self.cycles = 0;
                } else {
                    self.switch_mode(StatusMode::OamScan);
                    self.cycles = 0;
                }

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

    #[allow(clippy::too_many_lines)]
    fn draw_line(&mut self) {
        let line_start = SCREEN_WIDTH * (self.ly as usize);
        let line_end = SCREEN_WIDTH + line_start;

        let line_pixels = &mut self.framebuffer[line_start..line_end];
        let mut bg_priority = [false; SCREEN_WIDTH];

        let should_render_bg = self.lcdc.get_bg_enable();
        let should_render_win = self.lcdc.get_win_enable() && self.wy <= self.ly;

        if should_render_bg {
            let y = self.scy.wrapping_add(self.ly);
            let tile_row = (y / 8) as u16;

            for i in 0..(SCREEN_WIDTH as u8) {
                let x: u8 = self.scx.wrapping_add(i);
                let tile_col = (x / 8) as u16;

                let tile_address = {
                    let base_address = if self.lcdc.get_bg_map() {
                        0x9C00
                    } else {
                        0x9800
                    };

                    let mapped_address = base_address + (tile_row * 32 + tile_col);
                    let tile_index = self.vram.read(mapped_address) as u16;

                    if self.lcdc.get_bg_win_addr() {
                        // Unsigned mapping.
                        0x8000 + (tile_index * 16) // Each tile has 16 bytes.
                    } else {
                        // Signed mapping.
                        if tile_index < 128 {
                            0x9000 + (tile_index * 16)
                        } else {
                            0x8800 + ((tile_index - 128) * 16)
                        }
                    }
                };

                let line = ((y % 8) * 2) as u16;

                let data_lo = self.vram.read(tile_address + line);
                let data_hi = self.vram.read(tile_address + line + 1);

                let color_id = {
                    let bit = (x % 8) as usize;

                    let lo = ((data_lo << bit) >> 7) & 0b1;
                    let hi = ((data_hi << bit) >> 7) & 0b1;

                    (hi << 1) | lo
                };

                bg_priority[i as usize] = color_id != 0;
                line_pixels[i as usize] = Color::from_dmg_color_id_with_palette(color_id, self.bgp);
            }
        }

        if should_render_win {
            // Window X position is WX - 7.
            let window_x = self.wx.saturating_sub(7);

            let y = self.window_internal_counter as u16;
            let tile_row = y / 8;

            for i in window_x..(SCREEN_WIDTH as u8) {
                let x = {
                    let value = self.scx.wrapping_add(i);
                    if value >= window_x {
                        i - window_x
                    } else {
                        value
                    }
                };

                let tile_col = (x / 8) as u16;

                let tile_address = {
                    let base_address = if self.lcdc.get_win_map() {
                        0x9C00
                    } else {
                        0x9800
                    };

                    let mapped_address = base_address + (tile_row * 32 + tile_col);
                    let tile_index = self.vram.read(mapped_address) as u16;

                    if self.lcdc.get_bg_win_addr() {
                        // Unsigned mapping.
                        0x8000 + (tile_index * 16) // Each tile has 16 bytes.
                    } else {
                        // Signed mapping.
                        if tile_index < 128 {
                            0x9000 + (tile_index * 16)
                        } else {
                            0x8800 + ((tile_index - 128) * 16)
                        }
                    }
                };

                let line = (y % 8) * 2;

                let data_lo = self.vram.read(tile_address + line);
                let data_hi = self.vram.read(tile_address + line + 1);

                let color_id = {
                    let bit = (x % 8) as usize;

                    let lo = ((data_lo << bit) >> 7) & 0b1;
                    let hi = ((data_hi << bit) >> 7) & 0b1;

                    (hi << 1) | lo
                };

                bg_priority[i as usize] = color_id != 0;
                line_pixels[i as usize] = Color::from_dmg_color_id_with_palette(color_id, self.bgp);
            }
        }

        if self.lcdc.get_obj_enable() {
            let obj_height = if self.lcdc.get_obj_size() { 16 } else { 8 };
            let sprite_buffer = self.oam.get_sprites_in_line(self.ly, obj_height);

            // Since `sprite_buffer`'s priority is increasing, we need to reverse the iterator.
            for sprite in sprite_buffer.iter().rev() {
                let palette = if sprite.flags.contains(SpriteFlags::PALETTE_NUMBER) {
                    self.obp1
                } else {
                    self.obp0
                };

                let tile_index = {
                    if self.lcdc.get_obj_size() {
                        sprite.tile_index & !0x01
                    } else {
                        sprite.tile_index
                    }
                } as u16;

                // 16 bytes per tile.
                let base_tile_address = 0x8000 + (tile_index * 16);

                let tile_row = {
                    let row = self.ly.wrapping_sub(sprite.y) as u16;

                    if sprite.flags.contains(SpriteFlags::Y_FLIP) {
                        (obj_height as u16) - 1 - row
                    } else {
                        row
                    }
                };

                let tile_address = base_tile_address + (tile_row * 2);

                let data_lo = self.vram.read(tile_address);
                let data_hi = self.vram.read(tile_address + 1);

                for x in 0..=7 {
                    let color_id = {
                        let bit = if sprite.flags.contains(SpriteFlags::X_FLIP) {
                            7 - x
                        } else {
                            x
                        } as usize;

                        let lo = (data_lo >> bit) & 0b1;
                        let hi = (data_hi >> bit) & 0b1;

                        (hi << 1) | lo
                    };

                    let mapped_x = sprite.x.wrapping_add(7 - x) as usize;

                    if mapped_x < SCREEN_WIDTH && color_id != 0 {
                        if !sprite.flags.contains(SpriteFlags::PRIORITY) || !bg_priority[mapped_x] {
                            line_pixels[mapped_x] =
                                Color::from_dmg_color_id_with_palette(color_id, palette);
                        }
                    }
                }
            }
        }
    }
}

pub mod color;
mod debug_getters;
mod lcd_control;
mod lcd_status;
mod oam;
mod oam_dma;
mod registers;
mod sprite;
mod video_ram;
mod vram_dma;
