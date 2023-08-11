#![warn(clippy::pedantic, clippy::perf, clippy::all, clippy::complexity)]
#![allow(
    clippy::match_same_arms,
    clippy::cast_lossless,
    clippy::unused_self,
    clippy::similar_names,
    clippy::multiple_inherent_impl,
    clippy::enum_glob_use
)]
#![allow(
    clippy::cast_possible_truncation, // Intentional, but may be possible to mitigate.
    clippy::verbose_bit_mask // As per the docs, LLVM may not be able to generate better code.
)]

use constants::{Button, PALETTE};
use cpu::Cpu;

pub struct GameBoy {
    pub cpu: Cpu,
}

impl GameBoy {
    #[must_use]
    pub fn new() -> Self {
        let mut cpu = Cpu::default();

        #[cfg(not(feature = "bootrom"))]
        cpu.skip_bootrom();

        Self { cpu }
    }

    pub fn reset(&mut self) {
        *self = GameBoy::new();
    }

    pub fn load_cartridge(&mut self, rom: Vec<u8>) {
        self.reset();
        self.cpu.memory.load_cartridge(rom);
    }

    pub fn run_frame(&mut self) {
        self.cpu.cycles = 0;
        while self.cpu.cycles < (70224 * 2) {
            self.cpu.step();
        }
    }

    pub fn key_down(&mut self, key: Button) {
        self.cpu.memory.key_down(key);
    }

    pub fn key_up(&mut self, key: Button) {
        self.cpu.memory.key_up(key);
    }

    #[allow(clippy::identity_op)]
    pub fn draw(&self, frame: &mut [u8]) {
        let fb = self.cpu.memory.borrow_framebuffer();

        for (i, pixel) in fb.iter().enumerate() {
            frame[(i * 4) + 0] = PALETTE[*pixel as usize];
            frame[(i * 4) + 1] = PALETTE[*pixel as usize];
            frame[(i * 4) + 2] = PALETTE[*pixel as usize];
            frame[(i * 4) + 3] = 0xFF;
        }
    }
}

pub mod constants;

mod cartridge;
mod cpu;
mod graphics;
mod joypad;
mod memory;
mod serial;
mod sound;
mod timer;
