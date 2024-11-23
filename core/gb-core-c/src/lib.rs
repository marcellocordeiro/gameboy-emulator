use button::Button;
use gb_core::{
    GameBoy,
    constants::{DeviceModel, SCREEN_HEIGHT, SCREEN_WIDTH, ScreenPixels},
};
use types::{Bootrom, Rom, ToSlice as _};

#[no_mangle]
pub extern "C" fn gameboy_new(is_cgb: bool) -> *mut GameBoy {
    let device_model = if is_cgb {
        DeviceModel::Cgb
    } else {
        DeviceModel::Dmg
    };

    let gb = GameBoy::new(device_model);

    Box::into_raw(Box::new(gb))
}

/// # Safety
///
/// The memory for the Game Boy core has be allocated and valid
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
/// The Game Boy core pointer cannot be null.
#[no_mangle]
pub unsafe extern "C" fn gameboy_reset(gb_ptr: *mut GameBoy) {
    let gb = unsafe { &mut *gb_ptr };

    gb.reset();
}

/// # Safety
///
/// 1. The Game Boy core pointer cannot be null.
/// 2. The ROM array pointer cannot be null.
/// 3. The allocated size for the ROM has to be equal to `rom.size`.
/// 4. The bootrom is optional, but if provided its allocated size has to be equal to `bootrom.size`.
#[no_mangle]
pub unsafe extern "C" fn gameboy_load(gb_ptr: *mut GameBoy, bootrom: Bootrom, rom: Rom) -> bool {
    let gb = &mut *gb_ptr;

    let rom = unsafe { rom.to_slice().map(<[u8]>::to_vec) };
    let bootrom = unsafe { bootrom.to_slice().map(<[u8]>::to_vec) };

    let Some(rom) = rom else {
        return false;
    };

    gb.load(bootrom, rom).unwrap();

    true
}

/// # Safety
///
/// The Game Boy core pointer cannot be null.
#[no_mangle]
pub unsafe extern "C" fn gameboy_run_frame(gb_ptr: *mut GameBoy) {
    let gb = unsafe { &mut *gb_ptr };

    gb.run_frame();
}

/// # Safety
///
/// The Game Boy core pointer cannot be null.
#[no_mangle]
pub unsafe extern "C" fn gameboy_set_joypad_button(
    gb_ptr: *mut GameBoy,
    button: Button,
    value: bool,
) {
    let gb = unsafe { &mut *gb_ptr };

    gb.set_joypad_button(button.into(), value);
}

/// # Safety
///
/// The Game Boy core pointer cannot be null.
#[no_mangle]
pub unsafe extern "C" fn gameboy_joypad_button_up(gb_ptr: *mut GameBoy, button: Button) {
    let gb = unsafe { &mut *gb_ptr };

    gb.joypad_button_up(button.into());
}

/// # Safety
///
/// The Game Boy core pointer cannot be null.
#[no_mangle]
pub unsafe extern "C" fn gameboy_joypad_button_down(gb_ptr: *mut GameBoy, button: Button) {
    let gb = unsafe { &mut *gb_ptr };

    gb.joypad_button_down(button.into());
}

/// # Safety
///
/// 1. The Game Boy core pointer cannot be null.
/// 2. The frame array pointer cannot be null.
/// 3. The allocated size for the frame has to be equal to `SCREEN_WIDTH * SCREEN_HEIGHT * 4`.
#[no_mangle]
pub unsafe extern "C" fn gameboy_draw_into_frame_rgba8888(gb_ptr: *mut GameBoy, frame: *mut u8) {
    let gb = unsafe { &mut *gb_ptr };

    let slice: &mut ScreenPixels = unsafe {
        std::slice::from_raw_parts_mut(frame, SCREEN_WIDTH * SCREEN_HEIGHT * 4)
            .try_into()
            .unwrap_unchecked()
    };

    gb.draw_into_frame_rgba8888(slice);
}

pub mod button;
pub mod types;
