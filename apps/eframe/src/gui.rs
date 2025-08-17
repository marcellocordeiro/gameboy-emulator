use std::sync::mpsc::{Receiver, Sender};

use egui::{CentralPanel, Context, MenuBar, TopBottomPanel, ViewportCommand};
use gb_core::GameBoy;

use self::{
    control::Control,
    palettes::Palettes,
    screen_area::ScreenArea,
    state::State,
    tiles::Tiles,
};
use crate::{
    file_manager::{FileInfo, FileType, file_picker_async},
    gui::{audio::Audio, rom_drop_area::RomDropArea},
};

pub enum Event {
    BootromSelected(FileInfo),
    RomSelected(FileInfo),
}

pub struct Gui {
    pub event_receiver: Receiver<Event>,
    pub event_sender: Sender<Event>,

    pub audio: Audio,
    pub control: Control,
    pub palettes: Palettes,
    pub state: State,
    pub tiles: Tiles,
    pub screen_area: ScreenArea,
    //pub error: Option<Box<dyn Error>>,
    //pub message: Option<String>,
}

impl Gui {
    pub fn new(egui_ctx: &Context) -> Self {
        let (event_sender, event_receiver) = std::sync::mpsc::channel();

        Self {
            event_receiver,
            event_sender,
            audio: Audio::default(),
            control: Control::default(),
            palettes: Palettes::default(),
            state: State::default(),
            tiles: Tiles::new(egui_ctx),
            screen_area: ScreenArea::new(egui_ctx),
            //error: None,
            //message: None,
        }
    }

    pub fn render(&mut self, egui_ctx: &Context, gb_ctx: &mut GameBoy) {
        self.render_ui(egui_ctx, gb_ctx);
        self.render_main_area(egui_ctx, gb_ctx);
    }

    fn render_ui(&mut self, egui_ctx: &Context, gb_ctx: &mut GameBoy) {
        TopBottomPanel::top("top_panel").show(egui_ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Load ROM").clicked() {
                        file_picker_async(FileType::Rom, self.event_sender.clone());
                    }

                    if ui.button("Load bootrom").clicked() {
                        file_picker_async(FileType::Bootrom, self.event_sender.clone());
                    }

                    if ui.button("Reset").clicked() {
                        gb_ctx.reset();
                    }

                    if ui.button("Quit").clicked() {
                        egui_ctx.send_viewport_cmd(ViewportCommand::Close);
                    }
                });

                Control::draw_manual_control_button(self, ui);
                Control::draw_widget_toggle_button(self, ui);
                State::draw_widget_toggle_button(self, ui);
                Tiles::draw_widget_toggle_button(self, ui);
                Palettes::draw_widget_toggle_button(self, ui);
                Audio::draw_widget_toggle_button(self, ui);
            });
        });

        Control::draw(self, egui_ctx, gb_ctx);
        State::draw(self, egui_ctx, gb_ctx);
        Tiles::draw(self, egui_ctx, gb_ctx);
        Palettes::draw(self, egui_ctx, gb_ctx);
        Audio::draw(self, egui_ctx, gb_ctx);
    }

    fn render_main_area(&mut self, egui_ctx: &Context, gb_ctx: &GameBoy) {
        CentralPanel::default().show(egui_ctx, |ui| {
            if gb_ctx.cartridge_inserted() {
                ScreenArea::draw(self, ui, gb_ctx);
            } else {
                RomDropArea::draw(self, egui_ctx, ui);
            }
        });
    }
}

mod audio;
mod components;
mod control;
mod palettes;
mod rom_drop_area;
mod screen_area;
mod state;
mod tiles;
