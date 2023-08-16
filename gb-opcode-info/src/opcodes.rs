use serde::Deserialize;

#[derive(Deserialize)]
pub struct OpcodeTable {
    #[serde(rename = "Unprefixed")]
    pub unprefixed: Vec<Opcode>,

    #[serde(rename = "CBPrefixed")]
    pub cb_prefixed: Vec<Opcode>,
}

#[derive(Deserialize)]
pub struct Opcode {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Length")]
    pub length: i32,

    #[serde(rename = "TCyclesNoBranch")]
    pub tcycles_no_branch: i32,

    #[serde(rename = "TCyclesBranch")]
    pub tcycles_branch: i32,
}

pub fn parse_json() -> Result<OpcodeTable, serde_json::Error> {
    let data = include_str!("../../doc/generate_opcode_strings_rust/dmgops.json");
    let opcode_table: OpcodeTable = serde_json::from_str(data)?;

    Ok(opcode_table)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let _ = parse_json().unwrap();
    }
}
