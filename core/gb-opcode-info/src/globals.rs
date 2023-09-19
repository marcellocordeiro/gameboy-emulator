use std::sync::OnceLock;

use crate::opcodes::{parse_json, OpcodeTable};

static OPCODES: OnceLock<OpcodeTable> = OnceLock::new();

pub fn opcodes() -> &'static OpcodeTable {
    OPCODES.get_or_init(|| parse_json().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcodes_length() {
        let table = opcodes();

        assert_eq!(table.unprefixed.len(), 256);
        assert_eq!(table.cb_prefixed.len(), 256);
    }
}
