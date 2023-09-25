pub fn isolate_rightmost_bit(bits: u8) -> u8 {
    // Two's complement negation (-bits)
    let negated_bits = (!bits).wrapping_add(1);
    bits & negated_bits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isolate_rightmost_bit() {
        assert_eq!(isolate_rightmost_bit(0b1010), 0b0010);
        assert_eq!(isolate_rightmost_bit(0b1000), 0b1000);
        assert_eq!(isolate_rightmost_bit(0), 0);
        assert_eq!(isolate_rightmost_bit(0b1000_0000), 0b1000_0000);
        assert_eq!(isolate_rightmost_bit(0b1100_0000), 0b0100_0000);
    }
}
