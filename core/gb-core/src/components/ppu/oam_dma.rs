#[derive(Debug, Default, PartialEq, Eq)]
enum Status {
    #[default]
    Idle,
    Requested {
        base_source: u16,
    },
    Active {
        base_source: u16,
        offset: u16,
    },
    Restarting {
        next_base_source: u16,
        current_base_source: u16,
        current_offset: u16,
    },

    // Intermediate states
    ActiveFirstStep {
        base_source: u16,
    },
    RestartedFirstStep {
        base_source: u16,
    },
}

#[derive(Default)]
pub struct OamDma {
    dma: u8,
    status: Status,
}

impl OamDma {
    /// Used to check whether normal R/W to the OAM should be blocked or not.
    ///
    /// After a DMA write (when Idle), the first byte transfer is delayed by 1 M-cycle.
    /// However, because the emulator advances the DMA transfer before any reads or writes,
    /// we need an intermediate state between M == 1 and M == 2 that won't block the OAM.
    ///
    /// Restarting the OAM should keep the OAM blocked at all times, so we need
    /// another intermediate state while the DMA transfer is restarting.
    ///
    /// Write when Idle:
    /// - M == 0: DMA write, do nothing (OAM is accessible)
    /// - M == 1: do nothing (OAM is still accessible)
    /// - M == 2: transfer one byte (OAM is blocked)
    ///
    /// Write when Active (restarting):
    /// - M == 0: DMA write, previous DMA is running (OAM is blocked)
    /// - M == 1: previous DMA is running (OAM is blocked)
    /// - M == 2: new DMA starts (OAM is blocked)
    pub fn is_active(&self) -> bool {
        match self.status {
            Status::Idle => false,
            Status::Requested { .. } => false,
            Status::Active { .. } => true,
            Status::Restarting { .. } => true,

            Status::ActiveFirstStep { .. } => false,
            Status::RestartedFirstStep { .. } => true,
        }
    }

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

            Status::Requested { base_source } => {
                self.status = Status::ActiveFirstStep { base_source };

                None
            }

            Status::Active { offset: 0xA0, .. } => {
                self.status = Status::Idle;

                None
            }

            Status::Active {
                base_source,
                ref mut offset,
            } => {
                let source = base_source + *offset;
                let destination = 0xFE00 | *offset;

                *offset = offset.wrapping_add(1);

                Some((source, destination))
            }

            Status::Restarting {
                next_base_source,
                current_base_source,
                current_offset,
            } => {
                let source = current_base_source + current_offset;
                let destination = 0xFE00 | current_offset;

                self.status = Status::RestartedFirstStep {
                    base_source: next_base_source,
                };

                Some((source, destination))
            }

            Status::ActiveFirstStep { base_source }
            | Status::RestartedFirstStep { base_source } => {
                let source = base_source;
                let destination = 0xFE00;

                self.status = Status::Active {
                    base_source,
                    offset: 0x01,
                };

                Some((source, destination))
            }
        }
    }

    fn start(&mut self) {
        match self.status {
            Status::Idle | Status::Requested { .. } | Status::Restarting { .. } => {
                self.status = Status::Requested {
                    base_source: Self::base_source_address(self.dma),
                }
            }

            Status::Active {
                base_source,
                offset,
            } => {
                self.status = Status::Restarting {
                    next_base_source: Self::base_source_address(self.dma),
                    current_base_source: base_source,
                    current_offset: offset,
                }
            }

            Status::ActiveFirstStep { base_source }
            | Status::RestartedFirstStep { base_source } => {
                self.status = Status::Restarting {
                    next_base_source: Self::base_source_address(self.dma),
                    current_base_source: base_source,
                    current_offset: 0,
                }
            }
        }
    }

    fn base_source_address(dma: u8) -> u16 {
        let address = 0x100 * (dma as u16);

        if address < 0xE000 {
            address
        } else {
            // Mapped to the WRAM
            address - 0x2000
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_up() {
        let mut oam_dma = OamDma::default();

        assert_eq!(oam_dma.dma, 0);
        assert_eq!(oam_dma.status, Status::Idle);

        // Request
        oam_dma.write(0x00);
        assert_eq!(oam_dma.dma, 0);
        assert_eq!(oam_dma.status, Status::Requested { base_source: 0 });
        assert!(!oam_dma.is_active());

        assert_eq!(oam_dma.perform_dma(), None);
        assert_eq!(oam_dma.status, Status::ActiveFirstStep { base_source: 0 });
        assert!(!oam_dma.is_active());

        // The next read is expected to return a (source, destination)
        assert_eq!(oam_dma.perform_dma(), Some((0, 0xFE00)));
        assert_eq!(
            oam_dma.status,
            Status::Active {
                base_source: 0,
                offset: 0x01
            }
        );
        assert!(oam_dma.is_active());
    }

    #[test]
    fn test_dma_0x00() {
        let mut oam_dma = OamDma::default();

        assert_eq!(oam_dma.dma, 0);
        assert_eq!(oam_dma.status, Status::Idle);
        assert!(!oam_dma.is_active());

        // Request
        oam_dma.write(0);
        assert_eq!(oam_dma.dma, 0);
        assert_eq!(oam_dma.status, Status::Requested { base_source: 0 });
        assert!(!oam_dma.is_active());

        // First cycle (nothing is transferred)
        assert_eq!(oam_dma.perform_dma(), None);
        assert_eq!(oam_dma.status, Status::ActiveFirstStep { base_source: 0 });
        assert!(!oam_dma.is_active());

        // For the next 160 cycles
        for ((source, destination), offset) in (0..).zip(0xFE00..).zip(0..).take(0xA0) {
            if offset == 0 {
                assert_eq!(oam_dma.status, Status::ActiveFirstStep { base_source: 0 });
                assert!(!oam_dma.is_active());
            } else {
                assert_eq!(
                    oam_dma.status,
                    Status::Active {
                        base_source: 0,
                        offset
                    }
                );

                assert!(oam_dma.is_active());
            }

            assert_eq!(oam_dma.perform_dma(), Some((source, destination)));
        }

        // Done
        assert_eq!(oam_dma.perform_dma(), None);
        assert_eq!(oam_dma.status, Status::Idle);
        assert!(!oam_dma.is_active());
    }

    #[test]
    fn test_dma_0xff() {
        let mut oam_dma = OamDma::default();

        assert_eq!(oam_dma.dma, 0);
        assert_eq!(oam_dma.status, Status::Idle);
        assert!(!oam_dma.is_active());

        // Request
        oam_dma.write(0xFF);
        assert_eq!(oam_dma.dma, 0xFF);
        assert_eq!(
            oam_dma.status,
            Status::Requested {
                base_source: OamDma::base_source_address(0xFF)
            }
        );
        assert!(!oam_dma.is_active());

        // First cycle (nothing is transferred)
        assert_eq!(oam_dma.perform_dma(), None);
        assert_eq!(
            oam_dma.status,
            Status::ActiveFirstStep {
                base_source: (0xFF * 0x100) - 0x2000
            }
        );
        assert!(!oam_dma.is_active());

        // For the next 160 cycles
        for ((source, destination), offset) in ((0xFF * 0x100 - 0x2000)..)
            .zip(0xFE00..)
            .zip(0..)
            .take(0xA0)
        {
            if offset == 0 {
                assert_eq!(
                    oam_dma.status,
                    Status::ActiveFirstStep {
                        base_source: (0xFF * 0x100) - 0x2000
                    }
                );
                assert!(!oam_dma.is_active());
            } else {
                assert_eq!(
                    oam_dma.status,
                    Status::Active {
                        base_source: (0xFF * 0x100) - 0x2000,
                        offset
                    }
                );
                assert!(oam_dma.is_active());
            }

            assert_eq!(oam_dma.perform_dma(), Some((source, destination)));
        }

        // Done
        assert_eq!(oam_dma.perform_dma(), None);
        assert_eq!(oam_dma.status, Status::Idle);
        assert!(!oam_dma.is_active());
    }

    #[test]
    fn test_restart() {
        let mut oam_dma = OamDma::default();

        assert_eq!(oam_dma.dma, 0);
        assert_eq!(oam_dma.status, Status::Idle);
        assert!(!oam_dma.is_active());

        // Request
        oam_dma.write(0);
        assert_eq!(oam_dma.dma, 0);
        assert_eq!(oam_dma.status, Status::Requested { base_source: 0 });
        assert!(!oam_dma.is_active());

        // First cycle (nothing is transferred)
        assert_eq!(oam_dma.perform_dma(), None);
        assert_eq!(oam_dma.status, Status::ActiveFirstStep { base_source: 0 });
        assert!(!oam_dma.is_active());

        // For the next 2 cycles
        for ((source, destination), offset) in (0..).zip(0xFE00..).zip(0..).take(2) {
            if offset == 0 {
                assert_eq!(oam_dma.status, Status::ActiveFirstStep { base_source: 0 });
                assert!(!oam_dma.is_active());
            } else {
                assert_eq!(
                    oam_dma.status,
                    Status::Active {
                        base_source: 0,
                        offset
                    }
                );

                assert!(oam_dma.is_active());
            }

            assert_eq!(oam_dma.perform_dma(), Some((source, destination)));
        }

        // Verify
        assert_eq!(oam_dma.perform_dma(), Some((0x02, 0xFE02)));
        assert_eq!(
            oam_dma.status,
            Status::Active {
                base_source: 0,
                offset: 3
            }
        );
        assert!(oam_dma.is_active());

        // Restart at 0x02
        oam_dma.write(0x02);
        assert_eq!(oam_dma.dma, 0x02);
        assert_eq!(
            oam_dma.status,
            Status::Restarting {
                next_base_source: OamDma::base_source_address(0x02),
                current_base_source: 0,
                current_offset: 3
            }
        );
        assert!(oam_dma.is_active());

        // Next cycle
        assert_eq!(oam_dma.perform_dma(), Some((0x03, 0xFE03)));

        assert_eq!(oam_dma.perform_dma(), Some((0x02 * 0x100, 0xFE00)));
    }
}
