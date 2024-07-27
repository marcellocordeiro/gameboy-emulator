use std::sync::{mpsc, Arc};

use cartridge::{error::Error as CartridgeError, Cartridge};
pub use components::memory::MemoryInterface;
use components::{cpu::Cpu, mbc::MbcInterface, memory::Memory};
pub use constants::*;
pub use utils::{button::Button, color::Color};

pub struct GameBoy {
    cpu: Cpu,
    memory: Memory,

    cartridge: Option<Cartridge>,
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
            cartridge: None,
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
            self.memory.load_bootrom(bootrom);
        }

        if let Some(cartridge) = &self.cartridge {
            self.memory.load_cartridge(cartridge);

            if self.bootrom.is_none() {
                self.cpu.skip_bootrom();
                self.memory.skip_bootrom(cartridge);
            }
        }
    }

    pub fn load(&mut self, rom: Vec<u8>, bootrom: Option<Vec<u8>>) -> Result<(), CartridgeError> {
        if let Some(bootrom) = bootrom {
            self.insert_bootrom(bootrom);
        }

        self.insert_cartridge(rom)?;

        Ok(())
    }

    /// Insert the bootrom before the cartridge (TODO: improve this).
    fn insert_bootrom(&mut self, bootrom: Vec<u8>) {
        assert!(!self.cartridge_inserted());

        let bootrom = Arc::<Box<[u8]>>::from(bootrom.into_boxed_slice());
        self.memory.load_bootrom(bootrom.clone());

        self.bootrom = Some(bootrom);
    }

    /// Reset before inserting a new cartridge.
    fn insert_cartridge(&mut self, rom: Vec<u8>) -> Result<(), CartridgeError> {
        let cartridge = Cartridge::new(rom)?;

        self.memory.load_cartridge(&cartridge);

        if self.bootrom.is_none() {
            self.cpu.skip_bootrom();
            self.memory.skip_bootrom(&cartridge);
        }

        self.cartridge = Some(cartridge);

        Ok(())
    }

    pub fn cartridge_inserted(&self) -> bool {
        self.memory.mbc.is_some()
    }

    pub fn get_battery(&self) -> Option<&[u8]> {
        if let Some(mbc) = self.memory.mbc.as_ref() {
            return Some(mbc.get_battery());
        }

        None
    }

    pub fn load_battery(&mut self, file: Vec<u8>) {
        if let Some(mbc) = self.memory.mbc.as_mut() {
            mbc.load_battery(file);
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

    pub fn add_audio_callback(&mut self, callback: Box<components::apu::Callback>) {
        self.memory.apu.add_callback(callback);
    }
}

pub mod cartridge;
pub mod components;
mod constants;
mod utils;
