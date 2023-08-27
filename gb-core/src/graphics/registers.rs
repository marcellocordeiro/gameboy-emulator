use super::Graphics;

impl Graphics {
    pub fn read_scy(&self) -> u8 {
        self.scy
    }

    pub fn read_scx(&self) -> u8 {
        self.scx
    }

    pub fn read_ly(&self) -> u8 {
        self.ly
    }

    pub fn read_lyc(&self) -> u8 {
        self.lyc
    }

    pub fn read_bgp(&self) -> u8 {
        self.bgp
    }

    pub fn read_obp0(&self) -> u8 {
        self.obp0
    }

    pub fn read_obp1(&self) -> u8 {
        self.obp1
    }

    pub fn read_wy(&self) -> u8 {
        self.wy
    }

    pub fn read_wx(&self) -> u8 {
        self.wx
    }

    pub fn write_scy(&mut self, value: u8) {
        self.scy = value;
    }

    pub fn write_scx(&mut self, value: u8) {
        self.scx = value;
    }

    pub fn write_ly(&mut self, value: u8) {
        self.ly = value;
    }

    pub fn write_lyc(&mut self, value: u8) {
        self.lyc = value;
    }

    pub fn write_bgp(&mut self, value: u8) {
        self.bgp = value;
    }

    pub fn write_obp0(&mut self, value: u8) {
        self.obp0 = value;
    }

    pub fn write_obp1(&mut self, value: u8) {
        self.obp1 = value;
    }

    pub fn write_wy(&mut self, value: u8) {
        self.wy = value;
    }

    pub fn write_wx(&mut self, value: u8) {
        self.wx = value;
    }
}
