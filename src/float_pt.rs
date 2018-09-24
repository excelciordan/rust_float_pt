type Fpnum = u32;  // This is our Floating Point data type, note is is just an alias for u32

// This macro will cast a Fpnum as a f32
// Note, while a good exercise in creating a macro, it is no different than just calling 'as f32'
#[allow(unused_macros)]
macro_rules! asFpnum {
    ( x => $e:expr ) => ( $e as f32 )
}

#[allow(dead_code, unused_variables)]
fn fpadd(left:Fpnum, right:Fpnum, dest:&Fpnum) {

}

// Remeber that Floating Pt numbers are in ones compliment, therefore subtraction
// is the same as inverting the HO (sign) bit and adding
#[allow(dead_code, unused_variables)]
fn fpsub(left:Fpnum, right:Fpnum, dest:&Fpnum) {
    let temp_right = right ^ 0x80000000;  // Invert HO bit
    fpadd(left, temp_right, dest);
}

#[allow(dead_code, unused_variables)]
pub fn extract_sign( from:Fpnum ) -> u32 {
    return from >> 31;  // shift the HO bit to the LO bit
}

#[allow(dead_code, unused_variables)]
pub fn extract_exponent( from:Fpnum ) -> u32 {
    // let mask:u32 = 0xff; // Note that this is the same as 0x000000ff
    // let mut exponent:u32 = from >> 23; // drop mantisa
    // exponent = exponent & mask;
    // return exponent - 127

    // more efficient (it does not require more vars than mask)
    // Also, remember to account for excess-127
    // return ((from >> 23) & mask) - 127;
    
    // No vars added to stack
    return ((from >> 23) & 0xff) - 127; // no assignment
}

#[allow(dead_code, unused_variables)]
pub fn extract_mantisa( from:Fpnum ) -> u32 {
    // let mask:u32 = 0x711111; // 0b0000_0000_0111_1...1 (23 ones)
    // if (from & mask) == 0 {return 0}    // make sure to return 0 if zero
    // let implied_bit:u32 = 0x800000;   // implied 24 bit (always one)
    // return (from & mask) | implied_bit; // add back in the implied one
    
    // In 2 lines, no need to put new vars on the stack
    if (from & 0x7fffff) == 0 {return 0}
    return (from & 0x7fffff) | 0x8000000
}

#[allow(dead_code, unused_variables)]
fn shift_and_round(num_to_shift:Fpnum, num_shifts:i32) -> Fpnum {
    // Initial choice is i32 becuase if we implemnt shift left than the sign can be used as a direction indicator
    /* Oooo boy! 
        This is a dusey, REMEMBER shifting right is the same as dividing by 2.  We need to decrement the exp
        i.e. 1.345e3 -> 13.45e2 -> 134.5 e1

        Now for the fun part.  What we have here is a failure to communicate.  We don't want to just shift right
        and lose the bits we shift off.  We need to add those bits back in by rounding.  There are three rules

        1. Truncate the result if the last bit shifted out is a 0
        2. Bump the mantisa up by 1 if the last bit out was a 1 and there was at least one 1 among the other bits shifted out
        3. If the last bit out is the only 1 and the LO bit is 1, round up, else truncate

        Let the fun begin
    */


    return 1 as Fpnum;  // placeholder to make the ide happy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(2,2);
    }

    #[test]
    fn expect_four() {
        assert_eq!(4, 0x8 >> 1)
    }

    #[test]
    fn expect_one() {
        assert_eq!(1, (0x2 >> 1) & 0x1)
    }

    #[test]
    fn expect_zero() {
        assert_eq!(0, (0x5 >> 1) & 0x1)
    }

    #[test]
    #[should_panic]
    fn expect_one_in_ho_fail() {
        // 0x80000000 is defaulted to a i32 thus the answer is -1
        #[allow(overflowing_literals)]
        assert_eq!(1, 0x80000000 >> 31)
    }

    #[test]
    fn expect_one_in_ho() {
        assert_eq!(1, 0x80000000 as u32 >> 31)
    }

    #[test]
    #[should_panic]
    fn expect_b_is_cast_as_float() {
        let a:Fpnum = 0;
        let b = asFpnum!(x => a);
        assert_ne!(a as f32,b)
    }
}