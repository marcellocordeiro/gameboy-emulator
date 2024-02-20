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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
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
    pub cycles: Vec<Option<[String; 3]>>,
}

pub type Tests = Vec<Test>;

impl From<CpuState> for Registers {
    fn from(value: CpuState) -> Self {
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
            ime: ImeState::default(),
        }
    }
}

impl From<Registers> for CpuState {
    fn from(value: Registers) -> Self {
        Self {
            a: value.a,
            b: value.b,
            c: value.c,
            d: value.d,
            e: value.e,
            f: value.f.bits(),
            h: value.h,
            l: value.l,
            pc: value.pc,
            sp: value.sp,
        }
    }
}
