use gb_core::{Button, GameBoy, ScreenPixels, SCREEN_HEIGHT, SCREEN_WIDTH};

#[repr(C)]
pub enum GameBoyButton {
    A = 0,
    B = 1,
    SELECT = 2,
    START = 3,
    RIGHT = 4,
    LEFT = 5,
    UP = 6,
    DOWN = 7,
}

impl From<GameBoyButton> for Button {
    fn from(value: GameBoyButton) -> Self {
        match value {
            GameBoyButton::A => Self::A,
            GameBoyButton::B => Self::B,
            GameBoyButton::SELECT => Self::Select,
            GameBoyButton::START => Self::Start,
            GameBoyButton::RIGHT => Self::Right,
            GameBoyButton::LEFT => Self::Left,
            GameBoyButton::UP => Self::Up,
            GameBoyButton::DOWN => Self::Down,
        }
    }
}

#[no_mangle]
pub extern "C" fn gameboy_new() -> *mut GameBoy {
    Box::into_raw(Box::new(GameBoy::new()))
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
/// 3. The allocated size for the ROM has to be equal to `rom_size`.
#[no_mangle]
pub unsafe extern "C" fn gameboy_load_cartridge(
    gb_ptr: *mut GameBoy,
    rom: *const u8,
    rom_size: usize,
) {
    let gb = &mut *gb_ptr;

    let vec = unsafe { std::slice::from_raw_parts(rom, rom_size).to_vec() };

    gb.load_cartridge(vec).unwrap();
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
pub unsafe extern "C" fn gameboy_set_key(gb_ptr: *mut GameBoy, button: GameBoyButton, value: bool) {
    let gb = unsafe { &mut *gb_ptr };

    gb.set_key(Button::from(button), value);
}

/// # Safety
///
/// The Game Boy core pointer cannot be null.
#[no_mangle]
pub unsafe extern "C" fn gameboy_key_up(gb_ptr: *mut GameBoy, button: GameBoyButton) {
    let gb = unsafe { &mut *gb_ptr };

    gb.key_up(Button::from(button));
}

/// # Safety
///
/// The Game Boy core pointer cannot be null.
#[no_mangle]
pub unsafe extern "C" fn gameboy_key_down(gb_ptr: *mut GameBoy, button: GameBoyButton) {
    let gb = unsafe { &mut *gb_ptr };

    gb.key_down(Button::from(button));
}

/// # Safety
///
/// 1. The Game Boy core pointer cannot be null.
/// 2. The frame array pointer cannot*be null.
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
