use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    pub struct Flags: u8 {
        const ZERO       = 1 << 7;
        const N_ADD_SUB  = 1 << 6;
        const HALF_CARRY = 1 << 5;
        const CARRY      = 1 << 4;

        // Bits 0-3 are unused.
    }
}
