use std::sync::mpsc::Sender;

use gb_core::constants::{
    BOOTROM_EXTENSIONS,
    BOOTROM_EXTENSIONS_DESCRIPTION,
    ROM_EXTENSIONS,
    ROM_EXTENSIONS_DESCRIPTION,
};

use crate::{file_manager::FileInfo, gui::Event};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FileType {
    Bootrom,
    Rom,
}

pub fn file_picker_async(file_type: FileType, sender: Sender<Event>) {
    let (extensions_description, extensions) = match file_type {
        FileType::Bootrom => {
            (
                BOOTROM_EXTENSIONS_DESCRIPTION,
                BOOTROM_EXTENSIONS.as_slice(),
            )
        }
        FileType::Rom => (ROM_EXTENSIONS_DESCRIPTION, ROM_EXTENSIONS.as_slice()),
    };

    let task = rfd::AsyncFileDialog::new()
        .add_filter(extensions_description, extensions)
        .pick_file();

    execute(async move {
        let file_handle = task.await;

        let Some(file_handle) = file_handle else {
            return;
        };

        let data = file_handle.read().await.into();

        #[cfg(not(target_arch = "wasm32"))]
        let file_info = {
            let path = file_handle.path().to_path_buf();
            FileInfo { data, path }
        };

        #[cfg(target_arch = "wasm32")]
        let file_info = {
            let name = file_handle.file_name();
            FileInfo { data, name }
        };

        match file_type {
            FileType::Bootrom => {
                sender.send(Event::BootromSelected(file_info)).unwrap();
            }
            FileType::Rom => {
                sender.send(Event::RomSelected(file_info)).unwrap();
            }
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
    use pollster::FutureExt as _;
    std::thread::spawn(move || f.block_on());
}

#[cfg(target_arch = "wasm32")]
fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}
