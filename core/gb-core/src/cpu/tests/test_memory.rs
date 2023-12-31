use std::{cell::RefCell, collections::HashMap};

use crate::memory::{interrupts::Interrupts, speed_switch::SpeedSwitch, MemoryInterface};

#[derive(Default)]
pub struct TestMemory {
    pub data: HashMap<u16, u8>,
    pub logs: RefCell<Vec<[String; 3]>>,
}

impl MemoryInterface for TestMemory {
    fn tick(&mut self) {}

    fn read(&self, address: u16) -> u8 {
        let value = *self.data.get(&address).unwrap();

        self.logs.borrow_mut().push([
            format!("{address:#x}"),
            format!("{value:#x}"),
            "read".to_string(),
        ]);

        value
    }

    fn write(&mut self, address: u16, value: u8) {
        self.logs.borrow_mut().push([
            format!("{address:#x}"),
            format!("{value:#x}"),
            "write".to_string(),
        ]);

        self.data
            .entry(address)
            .and_modify(|e| *e = value)
            .or_insert(value);
    }

    fn speed_switch(&self) -> &SpeedSwitch {
        unimplemented!()
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
