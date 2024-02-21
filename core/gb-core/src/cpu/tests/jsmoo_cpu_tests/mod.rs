use std::path::PathBuf;

use self::{structs::Test, test_memory::TestMemory};
use crate::cpu::{tests::jsmoo_cpu_tests::structs::State, Cpu};

mod structs;
mod test_memory;

fn test_cpu(file_name: &str, test: &Test) {
    let mut cpu = Cpu::default();
    let mut memory = TestMemory::default();

    cpu.registers = test.initial.clone().into();
    memory.data = test.initial.ram.clone().into_iter().collect();

    cpu.step(&mut memory);

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

fn get_test_files() -> std::fs::ReadDir {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let path = format!("{manifest}/../../external/jsmoo-sm83-tests/v1");

    std::fs::read_dir(path).unwrap()
}

fn parse_test(path: &PathBuf) -> Vec<Test> {
    let json = std::fs::read_to_string(path).unwrap();

    serde_json::from_str(json.as_str()).unwrap()
}

#[test]
#[ignore = "todo"]
fn jsmoo_cpu_tests() {
    let files = get_test_files();
    let ignore = [
        "10", // STOP
        "76", // HALT
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
