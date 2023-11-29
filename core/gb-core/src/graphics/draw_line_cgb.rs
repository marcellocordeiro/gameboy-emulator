#![cfg(feature = "cgb")]

use crate::{constants::SCREEN_WIDTH, utils::color::Color};

use super::Graphics;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Priority {
    Object,
    OamAttribute,
    Background,
}

impl Graphics {
    pub fn draw_line_cgb(&mut self) {
        let mut priority = [Priority::Object; SCREEN_WIDTH];

        self.draw_tiles_cgb(&mut priority);
        self.draw_sprites_cgb(&priority);
    }

    #[allow(clippy::too_many_lines)]
    fn draw_tiles_cgb(&mut self, priority: &mut [Priority; SCREEN_WIDTH]) {
        let screen_line = {
            let line_offset = SCREEN_WIDTH * (self.ly as usize);

            &mut self.internal_screen.screen[line_offset..(line_offset + SCREEN_WIDTH)]
        };

        let should_render_win = self.lcdc.get_win_enable() && self.wy <= self.ly;
        let should_render_bg = self.cgb_mode || self.lcdc.get_bg_enable();

        let window_x = self.wx.saturating_sub(7);

        for i in 0..(SCREEN_WIDTH as u8) {
            let (x, y, tile_map_base_address) = if should_render_win && i >= window_x {
                let x = (i - window_x) as u16;
                let y = self.window_internal_counter as u16;

                let tile_map_base_address = if self.lcdc.get_win_map() {
                    0x9C00
                } else {
                    0x9800
                };

                (x, y, tile_map_base_address)
            } else if should_render_bg {
                let x = self.scx.wrapping_add(i) as u16;
                let y = self.scy.wrapping_add(self.ly) as u16;

                let tile_map_base_address = if self.lcdc.get_bg_map() {
                    0x9C00
                } else {
                    0x9800
                };

                (x, y, tile_map_base_address)
            } else {
                let pixel = Color::SYSTEM_DEFAULT;

                screen_line[i as usize] = pixel;

                continue;
            };

            let tile_map_address = {
                let tile_row = y / 8;
                let tile_col = x / 8;

                tile_map_base_address + ((tile_row * 32) + tile_col)
            };

            let tile_map_attributes = {
                if self.cgb_mode {
                    self.vram.read_bank_1(tile_map_address)
                } else {
                    0
                }
            };

            let (bg_oam_priority, y_flip, x_flip, in_bank_1, palette_number) = {
                // (0=Use OAM Priority bit, 1=BG Priority)
                let bg_oam_priority = (tile_map_attributes & 0b1000_0000) != 0;
                let y_flip = (tile_map_attributes & 0b0100_0000) != 0;
                let x_flip = (tile_map_attributes & 0b0010_0000) != 0;
                let in_bank_1 = (tile_map_attributes & 0b0000_1000) != 0;
                let palette_number = tile_map_attributes & 0b0000_0111;

                (bg_oam_priority, y_flip, x_flip, in_bank_1, palette_number)
            };

            let tile_address = {
                let tile_index = self.vram.read_bank_0(tile_map_address) as u16;

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

            let (tile_data_lo, tile_data_hi) = {
                let line = {
                    if y_flip {
                        14 - ((y % 8) * 2)
                    } else {
                        (y % 8) * 2
                    }
                };
                let base_address = tile_address + line;

                if in_bank_1 {
                    (
                        self.vram.read_bank_1(base_address),
                        self.vram.read_bank_1(base_address + 1),
                    )
                } else {
                    (
                        self.vram.read_bank_0(base_address),
                        self.vram.read_bank_0(base_address + 1),
                    )
                }
            };

            let color_id = {
                let bit = if x_flip { 7 - (x % 8) } else { x % 8 } as usize;

                let lo = ((tile_data_lo << bit) >> 7) & 0b1;
                let hi = ((tile_data_hi << bit) >> 7) & 0b1;

                (hi << 1) | lo
            };

            if self.cgb_mode {
                let raw_color = self.bg_cram.get_color_rgb555(palette_number, color_id);

                if color_id == 0 || !self.lcdc.get_bg_enable() {
                    priority[i as usize] = Priority::Object;
                } else if !bg_oam_priority {
                    priority[i as usize] = Priority::OamAttribute;
                } else {
                    priority[i as usize] = Priority::Background;
                }

                let pixel = Color::from_rgb555_u16_to_rgba8888(raw_color);
                screen_line[i as usize] = pixel;
            } else {
                let color_index = Color::apply_dmg_palette(color_id, self.bgp);
                let raw_color = self.bg_cram.get_color_rgb555(0, color_index);

                priority[i as usize] = if color_id == 0 {
                    Priority::Object
                } else {
                    Priority::Background
                };

                let pixel = Color::from_rgb555_u16_to_rgba8888(raw_color);
                screen_line[i as usize] = pixel;
            }
        }
    }

    fn draw_sprites_cgb(&mut self, priority: &[Priority; SCREEN_WIDTH]) {
        if !self.lcdc.get_obj_enable() {
            return;
        }

        let screen_line = {
            let line_offset = SCREEN_WIDTH * (self.ly as usize);

            &mut self.internal_screen.screen[line_offset..(line_offset + SCREEN_WIDTH)]
        };

        let obj_height = if self.lcdc.get_obj_size() { 16 } else { 8 };
        let sprite_buffer = if self.opri {
            self.oam
                .get_sprites_in_line_by_coordinate(self.ly, obj_height)
        } else {
            self.oam.get_sprites_in_line_by_oam(self.ly, obj_height)
        };

        // From lowest to highest priority.
        for sprite in sprite_buffer.iter().rev() {
            let selected_palette = if sprite.flags.obp1_selected {
                self.obp1
            } else {
                self.obp0
            };

            let tile_index = {
                if self.lcdc.get_obj_size() {
                    // 8x16
                    sprite.tile_index & !0b1
                } else {
                    // 8x8
                    sprite.tile_index
                }
            } as u16;

            let tile_address = {
                // 16 bytes per tile.
                let base_tile_address = 0x8000 + (tile_index * 16);

                let tile_row = {
                    let row = self.ly.wrapping_sub(sprite.y) as u16;

                    if sprite.flags.y_flip {
                        (obj_height as u16) - 1 - row
                    } else {
                        row
                    }
                };

                base_tile_address + (tile_row * 2)
            };

            let (tile_data_lo, tile_data_hi) = {
                if sprite.flags.in_bank_1 {
                    (
                        self.vram.read_bank_1(tile_address),
                        self.vram.read_bank_1(tile_address + 1),
                    )
                } else {
                    (
                        self.vram.read_bank_0(tile_address),
                        self.vram.read_bank_0(tile_address + 1),
                    )
                }
            };

            for x in 0..=7 {
                let color_id = {
                    let bit = if sprite.flags.x_flip { 7 - x } else { x } as usize;

                    let lo = (tile_data_lo >> bit) & 0b1;
                    let hi = (tile_data_hi >> bit) & 0b1;

                    (hi << 1) | lo
                };

                let mapped_x = sprite.x.wrapping_add(7 - x) as usize;

                if mapped_x >= SCREEN_WIDTH || color_id == 0 {
                    continue;
                }

                if self.cgb_mode {
                    if !(priority[mapped_x] == Priority::Object
                        || (priority[mapped_x] == Priority::OamAttribute
                            && !sprite.flags.bg_priority))
                    {
                        continue;
                    }

                    let raw_color = self
                        .obj_cram
                        .get_color_rgb555(sprite.flags.palette_number, color_id);

                    let pixel = Color::from_rgb555_u16_to_rgba8888(raw_color);

                    screen_line[mapped_x] = pixel;
                } else {
                    if priority[mapped_x] == Priority::Background && sprite.flags.bg_priority {
                        continue;
                    }

                    let color_index = Color::apply_dmg_palette(color_id, selected_palette);
                    let raw_color = self.obj_cram.get_color_rgb555(0, color_index);

                    let pixel = Color::from_rgb555_u16_to_rgba8888(raw_color);

                    screen_line[mapped_x] = pixel;
                }
            }
        }
    }
}
