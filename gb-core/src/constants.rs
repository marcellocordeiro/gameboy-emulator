use crate::graphics::color::Color;

pub const ONE_KIB: usize = 0x400;

pub const ONE_MIB_TO_KIB: usize = 1024;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

pub const FRAME_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT * core::mem::size_of::<u32>();

#[cfg(not(feature = "cgb"))]
pub const TILE_DATA_FRAME_WIDTH: usize = 128;

#[cfg(feature = "cgb")]
pub const TILE_DATA_FRAME_WIDTH: usize = 128 * 2;

pub const TILE_DATA_FRAME_HEIGHT: usize = 192;
pub const TILE_DATA_FRAME_SIZE: usize =
    TILE_DATA_FRAME_WIDTH * TILE_DATA_FRAME_HEIGHT * core::mem::size_of::<u32>();
pub const TILES_PER_LINE: usize = 16;

pub type Framebuffer = [Color; SCREEN_WIDTH * SCREEN_HEIGHT];
pub type Frame = [u8; FRAME_SIZE];
pub type TileDataFrame = [u8; TILE_DATA_FRAME_SIZE];

#[derive(Clone, Copy)]
pub enum Button {
    A = (1 << 0),
    B = (1 << 1),
    Select = (1 << 2),
    Start = (1 << 3),
    Right = (1 << 4),
    Left = (1 << 5),
    Up = (1 << 6),
    Down = (1 << 7),
}

impl Button {
    pub const fn to_array() -> [Self; 8] {
        use Button::*;
        [A, B, Select, Start, Right, Left, Up, Down]
    }
}
