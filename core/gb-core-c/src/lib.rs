use std::ffi::{c_uchar, c_ulonglong};

use gb_core::{GameBoy, ScreenPixels, SCREEN_HEIGHT, SCREEN_WIDTH};

#[no_mangle]
pub extern "C" fn gameboy_new() -> *mut GameBoy {
    Box::into_raw(Box::new(GameBoy::new()))
}

/// # Safety
///
/// The memory for the Game Boy core **has** be allocated and valid
/// to avoid double-free errors.
#[no_mangle]
pub unsafe extern "C" fn gameboy_destroy(gb_ptr: *mut GameBoy) {
    if gb_ptr.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(gb_ptr));
    }
}

/// # Safety
///
/// The Game Boy core pointer **cannot** be null.
#[no_mangle]
pub unsafe extern "C" fn gameboy_reset(gb_ptr: *mut GameBoy) {
    let gb = unsafe { &mut *gb_ptr };

    gb.reset();
}

/// # Safety
///
/// 1. The Game Boy core pointer **cannot** be null.
/// 2. The ROM array pointer **cannot** be null.
/// 3. The allocated size for the ROM **has** to be **equal** to `rom_size`.
#[no_mangle]
pub unsafe extern "C" fn gameboy_load_cartridge(
    gb_ptr: *mut GameBoy,
    rom: *const c_uchar,
    rom_size: c_ulonglong,
) {
    let gb = &mut *gb_ptr;

    let vec = unsafe { std::slice::from_raw_parts(rom, rom_size as usize).to_vec() };

    gb.load_cartridge(vec).unwrap();
}

/// # Safety
///
/// The Game Boy core pointer **cannot** be null.
#[no_mangle]
pub unsafe extern "C" fn gameboy_run_frame(gb_ptr: *mut GameBoy) {
    let gb = unsafe { &mut *gb_ptr };

    gb.run_frame();
}

/// # Safety
///
/// 1. The Game Boy core pointer **cannot** be null.
/// 2. The frame array pointer **cannot** be null.
/// 3. The allocated size for the frame **has** to be **equal** to `SCREEN_WIDTH * SCREEN_HEIGHT * 4`.
#[no_mangle]
pub unsafe extern "C" fn gameboy_draw_into_frame_rgba8888(
    gb_ptr: *mut GameBoy,
    frame: *mut c_uchar,
) {
    let gb = unsafe { &mut *gb_ptr };

    let slice: &mut ScreenPixels = unsafe {
        std::slice::from_raw_parts_mut(frame, SCREEN_WIDTH * SCREEN_HEIGHT * 4)
            .try_into()
            .unwrap_unchecked()
    };

    gb.draw_into_frame_rgba8888(slice);
}
