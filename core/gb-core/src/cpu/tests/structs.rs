use std::collections::HashMap;

use serde::Deserialize;

use super::deserializers::{deserialize_hex, deserialize_ram};
use crate::cpu::registers::{Flags, ImeState, Registers};

#[derive(Deserialize)]
pub struct State {
    pub cpu: CpuState,

    #[serde(deserialize_with = "deserialize_ram")]
    pub ram: HashMap<u16, u8>,
}

#[derive(Deserialize)]
pub struct CpuState {
    #[serde(deserialize_with = "deserialize_hex")]
    pub a: u8,

    #[serde(deserialize_with = "deserialize_hex")]
    pub b: u8,

    #[serde(deserialize_with = "deserialize_hex")]
    pub c: u8,

    #[serde(deserialize_with = "deserialize_hex")]
    pub d: u8,

    #[serde(deserialize_with = "deserialize_hex")]
    pub e: u8,

    #[serde(deserialize_with = "deserialize_hex")]
    pub f: u8,

    #[serde(deserialize_with = "deserialize_hex")]
    pub h: u8,

    #[serde(deserialize_with = "deserialize_hex")]
    pub l: u8,

    #[serde(deserialize_with = "deserialize_hex")]
    pub pc: u16,

    #[serde(deserialize_with = "deserialize_hex")]
    pub sp: u16,
}

#[derive(Deserialize)]
pub struct Test {
    pub name: String,
    pub initial: State,
    pub r#final: State,
    pub cycles: Vec<[String; 3]>,
}

pub type Tests = Vec<Test>;

impl Test {
    pub fn verify_trace(&self, trace: &[[String; 3]]) -> bool {
        self.cycles.iter().zip(trace.iter()).all(|(a, b)| a == b)
    }
}

impl State {
    pub fn verify_ram(&self, ram: &HashMap<u16, u8>) -> bool {
        self.ram == *ram
    }
}

impl CpuState {
    pub fn to_cpu_registers(&self) -> Registers {
        Registers {
            a: self.a,
            f: Flags::from_bits_truncate(self.f),
            b: self.b,
            c: self.c,
            d: self.d,
            e: self.e,
            h: self.h,
            l: self.l,
            pc: self.pc,
            sp: self.sp,
            ime: ImeState::default(),
        }
    }

    pub fn verify_cpu_registers(&self, registers: &Registers) -> bool {
        (self.a == registers.a)
            && (self.f == registers.f.bits())
            && (self.b == registers.b)
            && (self.c == registers.c)
            && (self.d == registers.d)
            && (self.e == registers.e)
            && (self.h == registers.h)
            && (self.l == registers.l)
            && (self.pc == registers.pc)
            && (self.sp == registers.sp)
    }
}
