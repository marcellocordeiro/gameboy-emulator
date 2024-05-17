pub const ONE_KIB: usize = 0x400;

pub const ONE_MIB_TO_KIB: usize = 1024;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

pub const SCREEN_PIXELS_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT * std::mem::size_of::<u32>();

// #[cfg(not(feature = "cgb"))]
// pub const TILE_DATA_FRAME_WIDTH: usize = 128;

// #[cfg(feature = "cgb")]
pub const TILE_DATA_FRAME_WIDTH: usize = 128 * 2;

pub const TILE_DATA_FRAME_HEIGHT: usize = 192;
pub const TILE_DATA_FRAME_SIZE: usize =
    TILE_DATA_FRAME_WIDTH * TILE_DATA_FRAME_HEIGHT * std::mem::size_of::<u32>();
pub const TILES_PER_LINE: usize = 16;

pub type ScreenPixels = [u8; SCREEN_PIXELS_SIZE];
pub type TileDataFrame = [u8; TILE_DATA_FRAME_SIZE];

pub const EXTENSIONS_DESCRIPTION: &str = "Game Boy/Game Boy Color ROM";
pub const EXTENSIONS: [&str; 2] = ["gb", "gbc"];

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum DeviceModel {
    #[default]
    Dmg,
    Cgb,
}
