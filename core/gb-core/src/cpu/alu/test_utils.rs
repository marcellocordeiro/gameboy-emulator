#![cfg_attr(feature = "sm83-test-data", cfg(test))]

use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
pub struct TestResult {
    #[serde(deserialize_with = "deserialize_hex_string")]
    pub value: u8,
    #[serde(deserialize_with = "deserialize_hex_string")]
    pub flags: u8,
}

#[derive(Deserialize)]
pub struct Test {
    #[serde(deserialize_with = "deserialize_hex_string")]
    pub x: u8,
    #[serde(deserialize_with = "deserialize_hex_string")]
    pub y: u8,
    #[serde(deserialize_with = "deserialize_hex_string")]
    pub flags: u8,
    pub result: TestResult,
}

pub type Tests = Vec<Test>;

pub fn deserialize_hex_string<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        String(String),
    }

    match StringOrInt::deserialize(deserializer)? {
        StringOrInt::String(s) => Ok(u8::from_str_radix(&s[2..], 16).unwrap()),
    }
}

pub fn parse_tests(name: &str) -> Tests {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let path = format!("{manifest}/../../external/sm83-test-data/alu_tests/v1/{name}.json");
    let json = std::fs::read_to_string(path).unwrap();

    serde_json::from_str::<Tests>(json.as_str()).unwrap()
}
