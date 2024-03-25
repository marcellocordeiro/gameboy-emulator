use std::sync::{mpsc, Arc};

use cartridge::error::Error as CartridgeError;
pub use constants::*;
use cpu::Cpu;
use log::error;
use memory::Memory;
pub use memory::MemoryInterface;
pub use utils::{button::Button, color::Color};

pub struct GameBoy {
    cpu: Cpu,
    memory: Memory,

    rom: Option<Arc<Box<[u8]>>>,
    bootrom: Option<Arc<Box<[u8]>>>,

    pub device_model: DeviceModel,
}

impl GameBoy {
    pub fn new(device_model: DeviceModel) -> Self {
        let cpu = Cpu::with_device_model(device_model);
        let memory = Memory::with_device_model(device_model);

        Self {
            cpu,
            memory,
            rom: None,
            bootrom: None,
            device_model,
        }
    }

    pub fn cpu(&self) -> &Cpu {
        &self.cpu
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn reset(&mut self) {
        self.cpu = Cpu::with_device_model(self.device_model);
        self.memory = Memory::with_device_model(self.device_model);

        if let Some(bootrom) = self.bootrom.clone() {
            self.memory.use_bootrom(bootrom);
        } else {
            self.cpu.skip_bootrom();
            self.memory.skip_bootrom();
        }

        if let Some(rom) = self.rom.clone() {
            let result = self.memory.load_cartridge(rom);

            if let Err(error) = result {
                error!("{error}");
            }
        }
    }

    /// Insert the bootrom before the cartridge (TODO: improve this).
    pub fn insert_bootrom(&mut self, bootrom: Option<Vec<u8>>) {
        assert!(!self.cartridge_inserted());

        let bootrom = if let Some(bootrom) = bootrom {
            let bootrom = Arc::<Box<[u8]>>::from(bootrom.into_boxed_slice());
            self.memory.use_bootrom(bootrom.clone());

            Some(bootrom)
        } else {
            self.cpu.skip_bootrom();
            self.memory.skip_bootrom();

            None
        };

        self.bootrom = bootrom;
    }

    /// Reset before inserting a new cartridge.
    pub fn insert_cartridge(&mut self, rom: Vec<u8>) -> Result<(), CartridgeError> {
        let rom = Arc::<Box<[u8]>>::from(rom.into_boxed_slice());

        self.memory.load_cartridge(rom.clone())?;
        self.rom = Some(rom);

        Ok(())
    }

    pub fn cartridge_inserted(&self) -> bool {
        self.memory.cartridge.is_some()
    }

    pub fn get_battery(&self) -> Option<&[u8]> {
        if let Some(cartridge) = self.memory.cartridge.as_ref() {
            return Some(cartridge.get_battery());
        }

        None
    }

    pub fn load_battery(&mut self, file: Vec<u8>) {
        if let Some(cartridge) = self.memory.cartridge.as_mut() {
            cartridge.load_battery(file);
        }
    }

    pub fn step(&mut self) {
        self.cpu.step(&mut self.memory);
    }

    pub fn run_frame(&mut self) {
        self.cpu.run_frame(&mut self.memory);
    }

    pub fn set_joypad_button(&mut self, button: Button, value: bool) {
        self.memory.joypad.set_joypad_button(button, value);
    }

    pub fn joypad_button_down(&mut self, key: Button) {
        self.memory.joypad.joypad_button_down(key);
    }

    pub fn joypad_button_up(&mut self, key: Button) {
        self.memory.joypad.joypad_button_up(key);
    }

    pub fn draw_into_frame_rgba8888(&self, frame: &mut ScreenPixels) {
        self.memory.ppu.screen().draw_into_frame_rgba8888(frame);
    }

    pub fn draw_into_frame_bgra8888(&self, frame: &mut ScreenPixels) {
        self.memory.ppu.screen().draw_into_frame_bgra8888(frame);
    }

    pub fn add_serial_channel(&mut self, channel: mpsc::Sender<u8>) {
        self.memory.serial.add_sender(channel);
    }

    pub fn add_audio_callback(&mut self, callback: Box<apu::Callback>) {
        self.memory.apu.add_callback(callback);
    }
}

mod apu;
pub mod cartridge;
mod constants;
mod cpu;
mod joypad;
mod memory;
mod ppu;
mod serial;
mod timer;
mod utils;
