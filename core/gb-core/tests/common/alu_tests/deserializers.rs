use serde::{Deserialize, Deserializer};

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
