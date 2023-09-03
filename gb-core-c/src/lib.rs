#![warn(clippy::pedantic, clippy::perf, clippy::all, clippy::complexity)]
#![allow(
    clippy::match_same_arms,
    clippy::cast_lossless,
    clippy::unused_self,
    clippy::similar_names,
    clippy::multiple_inherent_impl,
    clippy::enum_glob_use,
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::collapsible_if,
    clippy::new_without_default,
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation, // Intentional, but may be possible to mitigate.
    clippy::verbose_bit_mask, // As per the docs, LLVM may not be able to generate better code.
    clippy::cast_possible_wrap,
)]

use std::ffi::{c_uchar, c_ulonglong};

use gb_core::{
    constants::{Frame, HEIGHT, WIDTH},
    GameBoy,
};

#[no_mangle]
pub extern "C" fn gameboy_new() -> *mut GameBoy {
    Box::into_raw(Box::new(GameBoy::new()))
}

#[no_mangle]
pub unsafe extern "C" fn gameboy_reset(gb_ptr: *mut GameBoy) {
    let gb = unsafe { &mut *gb_ptr };

    gb.reset();
}

#[no_mangle]
pub unsafe extern "C" fn gameboy_load_cartridge(
    gb_ptr: *mut GameBoy,
    rom: *const c_uchar,
    rom_size: c_ulonglong,
) {
    let gb = &mut *gb_ptr;

    let vec = unsafe { std::slice::from_raw_parts(rom, rom_size as usize).to_vec() };

    gb.load_cartridge(vec).unwrap_unchecked();
}

#[no_mangle]
pub unsafe extern "C" fn gameboy_run_frame(gb_ptr: *mut GameBoy) {
    let gb = unsafe { &mut *gb_ptr };

    gb.run_frame();
}

#[no_mangle]
pub unsafe extern "C" fn gameboy_draw(gb_ptr: *mut GameBoy, frame: *mut c_uchar) {
    let gb = unsafe { &mut *gb_ptr };

    let slice: &mut Frame = unsafe {
        std::slice::from_raw_parts_mut(frame, WIDTH * HEIGHT * 4)
            .try_into()
            .unwrap_unchecked()
    };

    gb.draw(slice);
}
