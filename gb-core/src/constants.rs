pub const ONE_KIB: usize = 0x400;

pub const ONE_MIB_TO_KIB: usize = 1024;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

pub type Framebuffer = [u8; SCREEN_WIDTH * SCREEN_HEIGHT];
pub type Frame = [u8; SCREEN_WIDTH * SCREEN_HEIGHT * std::mem::size_of::<u32>()];

#[cfg(not(feature = "cgb"))]
pub const TILE_DATA_FRAME_WIDTH: usize = 128;

#[cfg(feature = "cgb")]
pub const TILE_DATA_FRAME_WIDTH: usize = 128 * 2;

pub const TILE_DATA_FRAME_HEIGHT: usize = 192;
pub const TILES_PER_LINE: usize = 16;
pub type TileDataFrame =
    [u8; TILE_DATA_FRAME_WIDTH * TILE_DATA_FRAME_HEIGHT * std::mem::size_of::<u32>()];

pub const PALETTE: [u8; 4] = [0xFF, 0xAA, 0x55, 0x00];

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
