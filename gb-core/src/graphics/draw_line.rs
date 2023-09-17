// TODO: leaving this here for now, but we don't need it anymore.

use crate::{constants::SCREEN_WIDTH, utils::color::Color};

use super::Graphics;

impl Graphics {
    #[cfg(not(feature = "cgb"))]
    #[allow(clippy::too_many_lines)]
    pub fn draw_line(&mut self) {
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

                let tile_map_address = {
                    let base_address = if self.lcdc.get_bg_map() {
                        0x9C00
                    } else {
                        0x9800
                    };

                    let tile_col = (x / 8) as u16;

                    base_address + ((tile_row * 32) + tile_col)
                };

                let tile_address = {
                    let tile_index = self.vram.read(tile_map_address) as u16;

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
                    let line = ((y % 8) * 2) as u16;
                    let base_address = tile_address + line;

                    (
                        self.vram.read(base_address),
                        self.vram.read(base_address + 1),
                    )
                };

                let color_id = {
                    let bit = (x % 8) as usize;

                    let lo = ((tile_data_lo << bit) >> 7) & 0b1;
                    let hi = ((tile_data_hi << bit) >> 7) & 0b1;

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

                let tile_map_address = {
                    let base_address = if self.lcdc.get_win_map() {
                        0x9C00
                    } else {
                        0x9800
                    };

                    let tile_col = (x / 8) as u16;

                    base_address + ((tile_row * 32) + tile_col)
                };

                let tile_address = {
                    let tile_index = self.vram.read(tile_map_address) as u16;

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
                    let line = (y % 8) * 2;
                    let base_address = tile_address + line;

                    (
                        self.vram.read(base_address),
                        self.vram.read(base_address + 1),
                    )
                };

                let color_id = {
                    let bit = (x % 8) as usize;

                    let lo = ((tile_data_lo << bit) >> 7) & 0b1;
                    let hi = ((tile_data_hi << bit) >> 7) & 0b1;

                    (hi << 1) | lo
                };

                bg_priority[i as usize] = color_id != 0;
                line_pixels[i as usize] = Color::from_dmg_color_id_with_palette(color_id, self.bgp);
            }
        }

        if self.lcdc.get_obj_enable() {
            let obj_height = if self.lcdc.get_obj_size() { 16 } else { 8 };
            let sprite_buffer = self
                .oam
                .get_sprites_in_line_by_coordinate(self.ly, obj_height);

            // Since `sprite_buffer`'s priority is increasing, we need to reverse the iterator.
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
                    (
                        self.vram.read(tile_address),
                        self.vram.read(tile_address + 1),
                    )
                };

                for x in 0..=7 {
                    let color_id = {
                        let bit = if sprite.flags.x_flip { 7 - x } else { x } as usize;

                        let lo = (tile_data_lo >> bit) & 0b1;
                        let hi = (tile_data_hi >> bit) & 0b1;

                        (hi << 1) | lo
                    };

                    let mapped_x = sprite.x.wrapping_add(7 - x) as usize;

                    if mapped_x < SCREEN_WIDTH && color_id != 0 {
                        if !sprite.flags.priority || !bg_priority[mapped_x] {
                            line_pixels[mapped_x] =
                                Color::from_dmg_color_id_with_palette(color_id, selected_palette);
                        }
                    }
                }
            }
        }
    }
}
