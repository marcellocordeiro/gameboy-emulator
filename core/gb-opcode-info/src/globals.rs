use std::sync::LazyLock;

use crate::opcodes::{OpcodeTable, parse_json};

static OPCODES: LazyLock<OpcodeTable> = LazyLock::new(|| parse_json().unwrap());

#[must_use]
pub fn opcodes() -> &'static OpcodeTable {
    &OPCODES
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
