use std::{ffi::OsStr, sync::Arc};

use egui::{Align2, Color32, DroppedFile, Sense, TextStyle};
use gb_core::constants::ROM_EXTENSIONS;
use tracing::info;

use crate::{
    file_manager::FileInfo,
    gui::{Event, Gui},
    sys::thread::spawn,
};

pub struct RomDropArea;

impl RomDropArea {
    pub fn draw(ctx: &Gui, ui: &mut egui::Ui) {
        Self::drop_zone(ui);

        ui.input(|i| {
            Self::handle_dropped_files(ctx, &i.raw.dropped_files);
        });
    }

    fn drop_zone(ui: &mut egui::Ui) {
        let rect = ui.content_rect();
        let _ = ui.allocate_rect(rect, Sense::empty());

        let file_name = ui.input(|i| {
            let path = i.raw.hovered_files.first()?.clone().path?;

            let extension = path.extension()?.to_str()?;

            if !path.is_file() && ROM_EXTENSIONS.contains(&extension) {
                None
            } else {
                Some(path.file_name().unwrap().display().to_string())
            }
        });

        if let Some(file_name) = file_name {
            let fill = if ui.visuals().dark_mode {
                Color32::from_black_alpha(192)
            } else {
                Color32::from_white_alpha(192)
            };

            ui.painter().rect_filled(rect, 0.0, fill);
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                file_name,
                TextStyle::Heading.resolve(ui.style()),
                ui.visuals().text_color(),
            );
        } else {
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                "Drop the Game Boy ROM here",
                TextStyle::Heading.resolve(ui.style()),
                ui.visuals().text_color(),
            );
        }
    }

    fn handle_dropped_files(
        ctx: &Gui,
        dropped_files: &[Arc<dyn DroppedFile + Send + Sync + 'static>],
    ) {
        let Some(file) = dropped_files.first().cloned() else {
            return;
        };

        let path = file.path().to_path_buf();
        let Some(extension) = path.extension().and_then(OsStr::to_str) else {
            info!("Unable to extract the file extension (no extension?)");
            return;
        };

        if !ROM_EXTENSIONS.contains(&extension) {
            info!("Path {path:?} with extension {extension:?} is not one of: {ROM_EXTENSIONS:?}");
            return;
        }

        let event_sender = ctx.event_sender.clone();

        spawn(async move {
            #[cfg(not(target_arch = "wasm32"))]
            let data = file.bytes().unwrap();

            #[cfg(target_arch = "wasm32")]
            let data = file.bytes_async().await.unwrap();

            let data = std::sync::Arc::from(data);
            let file_info = FileInfo { data, path };

            event_sender.send(Event::RomSelected(file_info)).unwrap();
        });
    }
}
