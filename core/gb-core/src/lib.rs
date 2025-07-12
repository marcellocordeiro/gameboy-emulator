use std::sync::{Arc, mpsc};

use components::{cartridge::error::CartridgeError, cpu::Cpu, memory::Memory};
use constants::{DeviceModel, ScreenPixels};
use utils::button::Button;

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

    pub fn reset(&mut self) {
        let audio_callback = self.memory.apu.take_callback();

        self.cpu = Cpu::with_device_model(self.device_model);
        self.memory = Memory::with_device_model(self.device_model);

        let Some(rom) = &self.rom else {
            return;
        };

        self.memory.load(self.bootrom.clone(), rom.clone()).unwrap();

        if self.bootrom.is_none() {
            self.cpu.skip_bootrom();
        }

        if let Some(callback) = audio_callback {
            self.add_audio_callback(callback);
        }
    }

    pub fn load(&mut self, bootrom: Option<Vec<u8>>, rom: Vec<u8>) -> Result<(), CartridgeError> {
        self.rom = Some(Arc::<Box<[u8]>>::from(rom.into_boxed_slice()));
        self.bootrom = bootrom.map(|vec| Arc::<Box<[u8]>>::from(vec.into_boxed_slice()));

        self.reset();

        Ok(())
    }

    pub fn cpu(&self) -> &Cpu {
        &self.cpu
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn memory_mut(&mut self) -> &mut Memory {
        &mut self.memory
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

    pub fn add_audio_callback(&mut self, callback: Box<components::apu::Callback>) {
        self.memory.apu.add_callback(callback);
    }
}

pub mod components;
pub mod constants;
pub mod error;
pub mod utils;
