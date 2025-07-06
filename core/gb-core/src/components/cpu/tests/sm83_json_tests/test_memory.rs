use super::structs::{Cycle, Ram};
use crate::components::memory::{
    MemoryInterface,
    interrupts::Interrupts,
    speed_switch::SpeedSwitch,
};

#[derive(Default)]
pub struct TestMemory {
    pub data: Ram,
    pub logs: Vec<Cycle>,

    pub bus_address: u16,
    pub bus_data: Option<u8>,

    pub interrupts: Interrupts,
}

impl MemoryInterface for TestMemory {
    fn force_cycle(&mut self) {
        self.logs
            .push((self.bus_address, self.bus_data, "---".to_string()));
    }

    fn read(&self, address: u16) -> u8 {
        *self.data.get(&address).unwrap()
    }

    fn write(&mut self, address: u16, value: u8) {
        self.data
            .entry(address)
            .and_modify(|e| *e = value)
            .or_insert(value);
    }

    fn read_cycle(&mut self, address: u16) -> u8 {
        let value = self.read(address);

        self.logs.push((address, Some(value), "r-m".to_string()));

        self.bus_address = address;
        self.bus_data = Some(value);

        value
    }

    fn write_cycle(&mut self, address: u16, value: u8) {
        self.write(address, value);

        self.logs.push((address, Some(value), "-wm".to_string()));

        self.bus_address = address;
        self.bus_data = Some(value);
    }

    fn speed_switch(&self) -> &SpeedSwitch {
        unimplemented!()
    }

    fn speed_switch_mut(&mut self) -> &mut SpeedSwitch {
        unimplemented!()
    }

    fn interrupts(&self) -> &Interrupts {
        &self.interrupts
    }

    fn interrupts_mut(&mut self) -> &mut Interrupts {
        &mut self.interrupts
    }
}
