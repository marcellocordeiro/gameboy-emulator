#[derive(Debug, Default, PartialEq, Eq)]
enum Status {
    #[default]
    Idle,
    Requested,
    Active,
}

#[derive(Default)]
pub struct OamDma {
    dma: u8,

    offset: u8,
    status: Status,
}

impl OamDma {
    pub fn read(&self) -> u8 {
        self.dma
    }

    pub fn write(&mut self, value: u8) {
        self.dma = value;
        self.start();
    }

    pub fn perform_dma(&mut self) -> Option<(u16, u16)> {
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

    fn start(&mut self) {
        self.offset = 0x00;
        self.status = Status::Requested;
    }

    fn stop(&mut self) {
        self.status = Status::Idle;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_up() {
        let mut oam_dma = OamDma::default();

        assert_eq!(oam_dma.dma, 0);
        assert_eq!(oam_dma.offset, 0);
        assert_eq!(oam_dma.status, Status::Idle);

        // Request
        oam_dma.write(0x00);
        assert_eq!(oam_dma.dma, 0);
        assert_eq!(oam_dma.offset, 0);
        assert_eq!(oam_dma.status, Status::Requested);

        assert_eq!(oam_dma.perform_dma(), None);

        // Next read is expected to return a (source, destination)
        let (source, destination) = oam_dma.perform_dma().expect("first read");
        assert_eq!(source, 0);
        assert_eq!(destination, 0xFE00);

        assert_eq!(oam_dma.dma, 0);
        assert_eq!(oam_dma.offset, 0x01);
        assert_eq!(oam_dma.status, Status::Active);
    }
}
