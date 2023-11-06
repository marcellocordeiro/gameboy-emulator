use std::path::PathBuf;

use gb_core::cpu::Cpu;

mod common;
use common::cpu_tests::{Test, TestMemory, Tests};

pub fn test_cpu(file_name: String, test: Test) {
    let mut cpu = Cpu::default();
    let mut test_memory = TestMemory::default();

    cpu.registers = test.initial.cpu.to_cpu_registers();
    test_memory.data = test.initial.ram.clone();

    cpu.step(&mut test_memory);

    assert!(
        test.r#final.cpu.verify_cpu_registers(&cpu.registers),
        "Test `{}` from `{file_name}` failed. The final registers do not match the expected result.",
        test.name
    );

    assert!(
        test.r#final.verify_ram(&test_memory.data),
        "Test `{}` from `{file_name}` failed. The final RAM does not match the expected result.",
        test.name
    );

    assert!(
        test.verify_trace(test_memory.logs.borrow().as_ref()),
        "Test `{}` from `{file_name}` failed. The trace does not match the expected result.",
        test.name
    );
}

pub fn get_test_files() -> std::fs::ReadDir {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let path = format!("{manifest}/../../external/sm83-test-data/cpu_tests/v1");

    let files = std::fs::read_dir(path).unwrap();

    files
}

pub fn parse_test(path: &PathBuf) -> Tests {
    let json = std::fs::read_to_string(path).unwrap();

    serde_json::from_str::<Tests>(json.as_str()).unwrap()
}

#[test]
fn test_00() {
    let files = get_test_files();
    let ignore = [
        "e7", "03", "c4", "76", "3b", "cc", "dc", "c5", "2b", "33", "cd", "23", "1b", "0b", "ef",
        "c7", "f7", "d4", "d5", "d7", "f5", "e5", "13", "df", "cf", "f9",
        //
        "10", // STOP tests
    ];

    for file in files {
        let file_path = file.unwrap().path();
        let file_name = file_path.file_stem().unwrap().to_str().unwrap().to_string();

        assert_eq!(file_path.extension().unwrap(), "json");

        if ignore.contains(&file_name.as_str()) {
            continue;
        }

        let tests = parse_test(&file_path);

        for test in tests {
            test_cpu(file_name.clone(), test);
        }
    }
}
