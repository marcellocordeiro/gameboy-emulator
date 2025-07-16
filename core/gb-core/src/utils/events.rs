use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Default)]
    pub struct Events: u8 {
        const VBLANK = 0b0000_0001;
    }
}
