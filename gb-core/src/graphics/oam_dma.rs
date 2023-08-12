#[derive(Default)]
enum Status {
    #[default]
    Idle,
    Requested,
    Active,
}

#[derive(Default)]
pub struct OamDma {
    pub dma: u8,
    offset: u8,
    status: Status,
}

impl OamDma {
    pub fn start(&mut self, value: u8) {
        self.dma = value;
        self.offset = 0x00;
        self.status = Status::Requested;
    }

    fn stop(&mut self) {
        self.status = Status::Idle;
    }

    pub fn advance(&mut self) -> Option<(u16, u16)> {
        match self.status {
            Status::Idle => None,

            Status::Requested => {
                self.status = Status::Active;

                None
            }

            Status::Active => {
                let source = {
                    let base_address = (self.dma as u16) << 8; // Same as * 0x100

                    if base_address < 0xE000 {
                        base_address + (self.offset as u16)
                    } else {
                        // Mapped to WRAM.
                        (base_address + (self.offset as u16)) - 0x2000
                    }
                };

                let destination = { 0xFE00 | (self.offset as u16) };

                self.offset = self.offset.wrapping_add(1);

                if self.offset == 0xA0 {
                    self.stop();
                }

                Some((source, destination))
            }
        }
    }
}
