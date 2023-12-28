use super::deserializers::deserialize_hex;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TestResult {
    #[serde(deserialize_with = "deserialize_hex")]
    pub value: u8,

    #[serde(deserialize_with = "deserialize_hex")]
    pub flags: u8,
}

#[derive(Deserialize)]
pub struct Test {
    #[serde(deserialize_with = "deserialize_hex")]
    pub x: u8,

    #[serde(deserialize_with = "deserialize_hex")]
    pub y: u8,

    #[serde(deserialize_with = "deserialize_hex")]
    pub flags: u8,

    pub result: TestResult,
}

pub type Tests = Vec<Test>;

pub fn parse_tests(name: &str) -> Tests {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let path = format!("{manifest}/../../external/sm83-test-data/alu_tests/v1/{name}.json");
    let json = std::fs::read_to_string(path).unwrap();

    serde_json::from_str::<Tests>(json.as_str()).unwrap()
}
