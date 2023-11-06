use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

pub fn deserialize_u8_hex_string<'de, D>(deserializer: D) -> Result<u8, D::Error>
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

pub fn deserialize_u16_hex_string<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        String(String),
    }

    match StringOrInt::deserialize(deserializer)? {
        StringOrInt::String(s) => Ok(u16::from_str_radix(&s[2..], 16).unwrap()),
    }
}

pub fn deserialize_ram_hashmap<'de, D>(deserializer: D) -> Result<HashMap<u16, u8>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        Vec(Vec<[String; 2]>),
    }

    Ok(match StringOrInt::deserialize(deserializer)? {
        StringOrInt::Vec(s) => {
            let mut h = HashMap::new();

            for e in s.iter() {
                let address = u16::from_str_radix(&e[0][2..], 16).unwrap();
                let value = u8::from_str_radix(&e[1][2..], 16).unwrap();
                h.insert(address, value);
            }

            h
        }
    })
}
