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
