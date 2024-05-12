use std::collections::BTreeMap;

use serde::Deserialize;

use crate::cpu::registers::{Flags, ImeState, Registers};

#[derive(Deserialize)]
pub struct Test {
    pub name: String,
    pub initial: State,
    pub r#final: State,
    pub cycles: Vec<Cycle>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct State {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
    pub ime: u8,
    pub ram: Vec<(u16, u8)>,
}

pub type Ram = BTreeMap<u16, u8>;
pub type Cycle = (u16, Option<u8>, String);

impl From<State> for Registers {
    fn from(value: State) -> Self {
        Self {
            a: value.a,
            f: Flags::from_bits_truncate(value.f),
            b: value.b,
            c: value.c,
            d: value.d,
            e: value.e,
            h: value.h,
            l: value.l,
            pc: value.pc,
            sp: value.sp,
            ime: if value.ime != 0 {
                ImeState::Enabled
            } else {
                ImeState::Disabled
            },
        }
    }
}

impl From<(Registers, Ram)> for State {
    fn from((registers, ram): (Registers, Ram)) -> Self {
        Self {
            a: registers.a,
            b: registers.b,
            c: registers.c,
            d: registers.d,
            e: registers.e,
            f: registers.f.bits(),
            h: registers.h,
            l: registers.l,
            pc: registers.pc,
            sp: registers.sp,
            ime: (registers.ime == ImeState::Enabled).into(),
            ram: ram.into_iter().collect(),
        }
    }
}
