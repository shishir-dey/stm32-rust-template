use core::assert;

/// Sets `n_bits` bits at `bit_position` in `value` to `new_bits_val`.
pub fn set_bits(value: u32, new_bits_val: u32, bit_position: u32, n_bits: u32) -> u32 {
    assert!(
        n_bits > 0 && n_bits <= 32,
        "n_bits must be between 1 and 32"
    );
    assert!(bit_position < 32, "bit_position must be less than 32");
    let mask = ((1 << n_bits) - 1) << bit_position;
    (value & !mask) | ((new_bits_val << bit_position) & mask)
}

/// Sets or clears a single bit at `bit_position` in `value` according to `bit_val`.
pub fn set_bit(value: u32, bit_position: u32, bit_val: bool) -> u32 {
    if bit_val {
        value | (1 << bit_position)
    } else {
        value & !(1 << bit_position)
    }
}

/// Reads the bit at `bit_position` in `value`.
pub fn read_bit(value: u32, bit_position: u32) -> bool {
    (value & (1 << bit_position)) != 0
}
