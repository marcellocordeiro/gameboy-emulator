use egui::{Align2, Color32, Context, DroppedFile, Sense, TextStyle, Ui};
use gb_core::constants::ROM_EXTENSIONS;

use crate::{
    file_manager::FileInfo,
    gui::{Event, Gui},
};

pub struct RomDropArea;

impl RomDropArea {
    pub fn draw(ctx: &Gui, egui_ctx: &Context, ui: &mut Ui) {
        Self::drop_zone(egui_ctx, ui);

        egui_ctx.input(|i| {
            Self::handle_dropped_files(ctx, &i.raw.dropped_files);
        });
    }

    fn drop_zone(ctx: &Context, ui: &mut Ui) {
        let rect = ctx.available_rect();
        let _ = ui.allocate_rect(rect, Sense::empty());

        let file_name = ctx.input(|i| {
            let path = i.raw.hovered_files.first()?.clone().path?;

            let extension = path.extension()?.to_str()?;

            if !path.is_file() && ROM_EXTENSIONS.contains(&extension) {
                None
            } else {
                Some(path.file_name().unwrap().display().to_string())
            }
        });

        if let Some(file_name) = file_name {
            ui.painter()
                .rect_filled(rect, 0.0, Color32::from_black_alpha(192));
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                file_name,
                TextStyle::Heading.resolve(&ctx.style()),
                Color32::WHITE,
            );
        } else {
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                "Drop the Game Boy ROM here",
                TextStyle::Heading.resolve(&ctx.style()),
                Color32::WHITE,
            );
        }
    }

    fn handle_dropped_files(ctx: &Gui, dropped_files: &[DroppedFile]) {
        let file_info = Self::first_valid_dropped_file(dropped_files);

        if let Some(file) = file_info {
            ctx.event_sender.send(Event::RomSelected(file)).unwrap();
        }
    }

    fn first_valid_dropped_file(dropped_files: &[DroppedFile]) -> Option<FileInfo> {
        let file = dropped_files.first()?;

        #[cfg(not(target_arch = "wasm32"))]
        {
            let path = file.path.clone()?;
            let extension = path.extension()?.to_str()?;

            if !path.is_file() && ROM_EXTENSIONS.contains(&extension) {
                None
            } else {
                let data = std::fs::read(&path).unwrap().into();
                Some(FileInfo { data, path })
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let name = file.name.clone();
            let data = file.bytes.clone()?;

            Some(FileInfo { data, name })
        }
    }
}
