pub const TITLE_ADDRESS_BEGIN: usize = 0x0134;
pub const TITLE_ADDRESS_END: usize = 0x0143;

pub fn get_title(encoded_title: &[u8]) -> String {
    String::from_utf8_lossy(encoded_title)
        .chars()
        .filter(char::is_ascii)
        .collect()
}
