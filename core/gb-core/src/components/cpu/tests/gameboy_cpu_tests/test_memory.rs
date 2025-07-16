use super::structs::{Cycle, Ram};
use crate::{
    components::{
        apu::Apu,
        memory::{MemoryInterface, interrupts::Interrupts, speed_switch::SpeedSwitch},
    },
    utils::events::Events,
};

#[derive(Default)]
pub struct TestMemory {
    pub data: Ram,
    pub logs: Vec<Option<Cycle>>,

    speed_switch: SpeedSwitch,
}

impl MemoryInterface for TestMemory {
    fn cycle(&mut self) {
        self.logs.push(None);
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

        self.logs
            .push(Some((address, Some(value), "read".to_string())));

        value
    }

    fn write_cycle(&mut self, address: u16, value: u8) {
        self.write(address, value);

        self.logs
            .push(Some((address, Some(value), "write".to_string())));
    }

    fn events(&self) -> &Events {
        unimplemented!()
    }

    fn events_mut(&mut self) -> &mut Events {
        unimplemented!()
    }

    fn process_speed_switch(&mut self) {
        unimplemented!()
    }

    fn apu(&self) -> &Apu {
        unimplemented!()
    }

    fn apu_mut(&mut self) -> &mut Apu {
        unimplemented!()
    }

    fn speed_switch(&self) -> &SpeedSwitch {
        &self.speed_switch
    }

    fn speed_switch_mut(&mut self) -> &mut SpeedSwitch {
        unimplemented!()
    }

    fn interrupts(&self) -> &Interrupts {
        unimplemented!()
    }

    fn interrupts_mut(&mut self) -> &mut Interrupts {
        unimplemented!()
    }
}
