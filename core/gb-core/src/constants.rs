pub const ONE_KIB: usize = 0x400;
pub const ONE_MIB_TO_KIB: usize = 1024;

pub const CPU_CLOCK_RATE: usize = 4194304;
pub const CPU_APPROX_M_CYCLES_PER_FRAME: usize = 70224;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

pub const SCREEN_PIXELS_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT * size_of::<u32>();

pub const TILE_DATA_FRAME_WIDTH_DMG: usize = 128;
pub const TILE_DATA_FRAME_WIDTH_CGB: usize = TILE_DATA_FRAME_WIDTH_DMG * 2;

pub const TILE_DATA_FRAME_HEIGHT: usize = 192;
pub const TILE_DATA_FRAME_SIZE_CGB: usize =
    TILE_DATA_FRAME_WIDTH_CGB * TILE_DATA_FRAME_HEIGHT * size_of::<u32>();
pub const TILES_PER_LINE: usize = 16;

pub type ScreenPixels = [u8; SCREEN_PIXELS_SIZE];
pub type TileDataFrameCgb = [u8; TILE_DATA_FRAME_SIZE_CGB];

pub const EXTENSIONS_DESCRIPTION: &str = "Game Boy/Game Boy Color ROM";
pub const EXTENSIONS: [&str; 2] = ["gb", "gbc"];

pub const BATTERY_EXTENSIONS_DESCRIPTION: &str = "Game Boy/Game Boy Color Battery";
pub const BATTERY_EXTENSIONS: [&str; 1] = ["sav"];

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum DeviceModel {
    #[default]
    Dmg,
    Cgb,
}

impl DeviceModel {
    pub fn is_cgb(self) -> bool {
        self == Self::Cgb
    }
}
