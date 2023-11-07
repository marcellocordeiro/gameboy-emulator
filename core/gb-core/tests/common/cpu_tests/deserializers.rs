use std::collections::HashMap;

use num::Unsigned;
use serde::{Deserialize, Deserializer};

pub fn deserialize_hex<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Unsigned,
    T::FromStrRadixErr: std::fmt::Debug,
    D: Deserializer<'de>,
{
    let hex_string = String::deserialize(deserializer)?;

    Ok(T::from_str_radix(&hex_string[2..], 16).unwrap())
}

pub fn deserialize_ram<'de, D>(deserializer: D) -> Result<HashMap<u16, u8>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Entries {
        Entries(Vec<[String; 2]>),
    }

    Ok(match Entries::deserialize(deserializer)? {
        Entries::Entries(entries) => entries
            .into_iter()
            .map(|[address, value]| {
                (
                    u16::from_str_radix(&address[2..], 16).unwrap(),
                    u8::from_str_radix(&value[2..], 16).unwrap(),
                )
            })
            .collect::<HashMap<u16, u8>>(),
    })
}
