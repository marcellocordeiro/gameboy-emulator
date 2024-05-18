use std::{ffi::OsStr, path::PathBuf};

use itertools::Itertools;

use self::{structs::Test, test_memory::TestMemory};
use crate::components::{
    cpu::{tests::gameboy_cpu_tests::structs::State, Cpu},
    memory::MemoryInterface,
};

mod structs;
mod test_memory;

fn test_cpu(file_name: &str, test: &Test) {
    let mut cpu = Cpu::default();
    let mut memory = TestMemory::default();

    cpu.registers = test.initial.clone().into();
    memory.data = test.initial.ram.clone().into_iter().collect();

    cpu.registers.pc = cpu.registers.pc.wrapping_sub(1);

    cpu.step(&mut memory);
    memory.logs.remove(0);
    let _ = memory.read_cycle(cpu.registers.pc);

    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);

    assert_eq!(
        State::from((cpu.registers, memory.data)),
        test.r#final,
        "Test `{}` from `{file_name}` failed. The final state do not match the expected result.",
        test.name
    );

    assert_eq!(
        *memory.logs, test.cycles,
        "Test `{}` from `{file_name}` failed. The trace does not match the expected result.",
        test.name
    );
}

fn get_test_files() -> Vec<std::fs::DirEntry> {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let path = format!("{manifest}/../../external/GameboyCPUTests/v2");

    std::fs::read_dir(path)
        .unwrap()
        .filter_map(|file| {
            let file = file.unwrap();

            (file.path().extension() == Some(OsStr::new("json"))).then_some(file)
        })
        .sorted_by(|a, b| a.path().cmp(&b.path()))
        .collect()
}

fn parse_test(path: &PathBuf) -> Vec<Test> {
    let json = std::fs::read_to_string(path).unwrap();

    serde_json::from_str(json.as_str()).unwrap()
}

#[test]
fn gameboy_cpu_tests() {
    let files = get_test_files();

    for file in files {
        let file_path = file.path();
        let file_name = file_path.file_stem().unwrap().to_str().unwrap();

        let tests = parse_test(&file_path);

        for test in tests {
            test_cpu(file_name, &test);
        }
    }
}
