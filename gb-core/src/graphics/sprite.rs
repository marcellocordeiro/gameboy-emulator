use bitflags::bitflags;

bitflags!(
    #[derive(Default, Clone, Copy)]
    pub struct SpriteFlags: u8 {
        const PRIORITY       = (1 << 7);
        const Y_FLIP         = (1 << 6);
        const X_FLIP         = (1 << 5);
        const PALETTE_NUMBER = (1 << 4);
    }
);

#[derive(Default)]
pub struct SpriteObject {
    pub y: u8, // Vertical position + 16.
    pub x: u8, // Horizontal position + 8.
    pub tile_index: u8,
    pub flags: SpriteFlags,
}
