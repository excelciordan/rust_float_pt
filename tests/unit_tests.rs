extern crate floatpt;

#[test]
fn test_extact_sign() {
    assert_eq!(1, floatpt::float_pt::extract_sign(0x80000000 as u32));
}

#[test]
fn test_extract_exponent() {
    // Don't forget the leading 0 .. 0011_1111_1000_0...0 -> 3f8
    assert_eq!(0, floatpt::float_pt::extract_exponent(0x3f800000 as u32));
}

#[test]
fn test_extract_mantisa() {
    assert_eq!(0x80000f as u32, floatpt::float_pt::extract_mantisa(0x0000000f as u32));
}

#[test]
fn test_shift_and_round_lo_zero() {
    let mut num_to_shift: u32 = 0xf01000;
    let num_shifts: u32 = 20;
    let num_shifted = num_to_shift >> num_shifts;
    floatpt::float_pt::shift_and_round(&mut num_to_shift, &num_shifts);
    assert_eq!(num_shifted, num_to_shift);
}

#[test]
fn test_shift_and_round_lo_one_last_bit_shifted_one() {
    let mut num_to_shift: u32 = 0xf80000;
    let num_shifts: u32 = 20;
    let num_shifted = (num_to_shift >> num_shifts) + 1;
    floatpt::float_pt::shift_and_round(&mut num_to_shift, &num_shifts);
    assert_eq!(num_shifted, num_to_shift);
}

#[test]
fn test_shift_and_round_lo_one_last_bit_shifted_zero() {
    let mut num_to_shift: u32 = 0xf70000;
    let num_shifts: u32 = 20;
    let num_shifted = num_to_shift >> num_shifts;
    floatpt::float_pt::shift_and_round(&mut num_to_shift, &num_shifts);
    assert_eq!(num_shifted, num_to_shift);
}

#[test]
fn test_shift_and_round_last_bit_shifted_one_with_another_one() {
    let mut num_to_shift: u32 = 0xe80000;
    let num_shifts: u32 = 20;
    let num_shifted = num_to_shift >> num_shifts;
    floatpt::float_pt::shift_and_round(&mut num_to_shift, &num_shifts);
    assert_eq!(num_shifted, num_to_shift);
}