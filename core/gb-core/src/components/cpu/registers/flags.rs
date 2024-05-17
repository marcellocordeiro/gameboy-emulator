use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    pub struct Flags: u8 {
        const ZERO = 1 << 7;
        const N_ADD_SUB = 1 << 6;
        const HALF_CARRY = 1 << 5;
        const CARRY = 1 << 4;

        // Bits 0-3 are unused.
    }
}

impl Flags {
    pub fn zero(&self) -> bool {
        self.contains(Self::ZERO)
    }

    pub fn n_add_sub(&self) -> bool {
        self.contains(Self::N_ADD_SUB)
    }

    pub fn half_carry(&self) -> bool {
        self.contains(Self::HALF_CARRY)
    }

    pub fn carry(&self) -> bool {
        self.contains(Self::CARRY)
    }
}
