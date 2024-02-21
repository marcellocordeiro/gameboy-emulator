use std::path::PathBuf;

use self::{
    structs::{Test, Tests},
    test_memory::TestMemory,
};
use crate::cpu::{tests::sm83_test_data::structs::CpuState, Cpu};

mod deserializers;
mod structs;
mod test_memory;

fn test_cpu(file_name: &str, test: &Test) {
    let mut cpu = Cpu::default();
    let mut memory = TestMemory::default();

    cpu.registers = test.initial.cpu.clone().into();
    memory.data = test.initial.ram.clone();

    cpu.step(&mut memory);

    assert_eq!(
        CpuState::from(cpu.registers),
        test.r#final.cpu,
        "Test `{}` from `{file_name}` failed. The final registers do not match the expected result.",
        test.name
    );

    assert_eq!(
        memory.data, test.r#final.ram,
        "Test `{}` from `{file_name}` failed. The final RAM does not match the expected result.",
        test.name
    );

    assert_eq!(
        *memory.logs, test.cycles,
        "Test `{}` from `{file_name}` failed. The trace does not match the expected result.",
        test.name
    );
}

fn get_test_files() -> std::fs::ReadDir {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let path = format!("{manifest}/../../external/sm83-test-data/cpu_tests/v1");

    std::fs::read_dir(path).unwrap()
}

fn parse_test(path: &PathBuf) -> Tests {
    let json = std::fs::read_to_string(path).unwrap();

    serde_json::from_str::<Tests>(json.as_str()).unwrap()
}

#[test]
fn sm83_test_data_cpu_tests() {
    let files = get_test_files();
    let ignore = [
        "e7", "03", "c4", "76", "3b", "cc", "dc", "c5", "2b", "33", "cd", "23", "1b", "0b", "ef",
        "c7", "f7", "d4", "d5", "d7", "f5", "e5", "13", "df", "cf", "f9",
        //
        "10", // STOP
    ];

    for file in files {
        let file_path = file.unwrap().path();
        let file_name = file_path.file_stem().unwrap().to_str().unwrap();

        assert_eq!(file_path.extension().unwrap(), "json");

        if ignore.contains(&file_name) {
            continue;
        }

        let tests = parse_test(&file_path);

        for test in tests {
            test_cpu(file_name, &test);
        }
    }
}
