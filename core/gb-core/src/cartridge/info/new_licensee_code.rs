pub const NEW_LICENSEE_CODE_ADDRESS_BEGIN: usize = 0x0144;
pub const NEW_LICENSEE_CODE_ADDRESS_END: usize = 0x0145;

pub fn get_new_licensee_code(encoded_code: &[u8]) -> String {
    String::from_utf8_lossy(encoded_code)
        .trim()
        .chars()
        .filter(char::is_ascii)
        .collect()
}
