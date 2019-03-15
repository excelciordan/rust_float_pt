type Fpnum = u32;  // This is our Floating Point data type, note is is just an alias for u32

// This macro will cast a Fpnum as a f32
// Note, while a good exercise in creating a macro, it is no different than just calling 'as f32'
#[allow(unused_macros)]
macro_rules! asFpnum {
    ( x => $e:expr ) => ( $e as f32 )
}

#[allow(unused_variables)]
pub fn fpadd(left:Fpnum, right:Fpnum, dest:&mut Fpnum) {
    // This function takes some work
    // 1. split numbers into components (sign, mantisa, expontnet)
    // 2. Determin larger exponent
    // 3. get the shifted value of the larger exonent (don't forget to update the expnent)
    // 4. add if sign bits are the same, otherwise subtract
    // 5. shift exponent back to 1.xxx eNN
    // 6. build proper fpnum for the mutable entry

}

// Remeber that Floating Pt numbers are in ones compliment, therefore subtraction
// is the same as inverting the HO (sign) bit and adding
pub fn fpsub(left:Fpnum, right:Fpnum, dest:&mut Fpnum) {
    // let temp_right = right ^ 0x80000000;  // Invert HO bit,  uneeded var
    fpadd(left, right ^ (0x80000000 as u32), dest);
}

pub fn extract_sign( from:Fpnum ) -> u32 {
    return from >> 31;  // shift the HO bit to the LO bit
}

pub fn extract_exponent( from:Fpnum ) -> u32 {
    // let mask:u32 = 0xff; // Note that this is the same as 0x000000ff
    // let mut exponent:u32 = from >> 23; // drop mantisa
    // exponent = exponent & mask;
    // return exponent - 127

    // more efficient (it does not require more vars than mask)
    // Also, remember to account for excess-127
    // return ((from >> 23) & mask) - 127;
    
    // No vars added to stack
    return ((from >> 23) & 0xff as u32) - 127; // no assignment
}

pub fn extract_mantisa( from:Fpnum ) -> u32 {
    // let mask:u32 = 0x711111; // 0b0000_0000_0111_1...1 (23 ones)
    // if (from & mask) == 0 {return 0}    // make sure to return 0 if zero
    // let implied_bit:u32 = 0x800000;   // implied 24 bit (always one)
    // return (from & mask) | implied_bit; // add back in the implied one
    
    // In 2 lines, no need to put new vars on the stack
    if (from & 0x7fffff as u32) == 0 {return 0}
    return (from & 0x7fffff as u32) | 0x800000
}

pub fn shift_and_round(num_to_shift:&mut Fpnum, num_shifts:&Fpnum) {
    /*  This is the my attempt at the author's solution 
        It is a very good solution, but it has a couple if statemets to many, also, the
        authors solution points out that I miss understood the 3 conditions, specifcally
        the second one.
    */
    
    //mask bits to check for a "sticky" bit
    let masks:[Fpnum; 24] = [
        0, 1, 3, 7, 0xf, 0x1f, 0x3f, 0x7f,
        0xff, 0x1ff, 0x3ff, 0x7ff, 0xfff, 0x1fff, 0x3fff, 0x7fff,
        0xffff, 0x1ffff, 0x3ffff, 0x7ffff, 0xfffff, 0x1fffff, 0x3fffff,
        0x7fffff
    ];

    //HO masks out the the HO bit of the value masked by the masks entry
    let ho_masks:[Fpnum; 24] = [
        0x0, 0x1, 0x2, 0x4, 0x8, 0x10, 0x20, 0x40, 0x80,
        0x100, 0x200, 0x400, 0x800, 0x1000, 0x2000, 0x4000,
        0x8000, 0x10000, 0x20000, 0x40000, 0x80000, 0x100000,
        0x200000, 400000
    ];

    // Shifted out holds the value that will be shifted out of the mantisa
    let shifted_out:u32 = *num_to_shift & masks[*num_shifts as usize];
    assert!(*num_shifts <= 23); //Make sure that the shifts won't make the number 0

    *num_to_shift = *num_to_shift >> *num_shifts;

    /* Okay, quick Rust aside, indexies of arrays or slices are of type usize
        usize is the number of bytes it takes to reference any pint in memory
        this means that if your process is 64bit, the usize is 8 bytes and
        similarly 4bytes for a 32 bit processor

        num_shits is a ref to a 32 bit unsigned number, hence we need to cast
        as a usize
     */
    // If decimal is greater than half way to the next num, round
    if shifted_out > ho_masks[*num_shifts as usize] {
        *num_to_shift += 1;
    }

    // If it is half way, round up if lo bit of shifted is also 1
    if shifted_out == ho_masks[*num_shifts as usize]{
        *num_to_shift += *num_to_shift & 1;  // compare LO bit to 1
    }
}

pub fn shift_and_round_my_solution(num_to_shift: &Fpnum, num_shifts: &u32) -> Fpnum {
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

        This is my solution to the problem.  The auther of write great code instead
        uses an array of masks.  That way he skips the bit shifts and just does the &.

        In "modern" processors, there is probably not a big diff in time, however, not
        all processors have a barrel shifter (multiple bits shifted in one op) so this
        solution is good, but not great
    */

    assert!(*num_shifts <= 23); //Make sure that the shifts won't make the number 0

    // We want the last bit shifted out, not the lo bit, hence the -1
    if (num_to_shift >> num_shifts - 1) & 1 == 0 {
        return num_to_shift >> num_shifts;
    }

    for i in 0..num_shifts - 2 { // we only care about the bits before the last shifted out
        if (num_to_shift >> i) & 1 == 1 {
            return (num_to_shift >> num_shifts) + 1;
        }
    }

    if (num_to_shift >> num_shifts) & 1 == 1 { // the LO bit is 1 and the last_out is 1
        return (num_to_shift >> num_shifts) + 1;
    }
    return num_to_shift >> num_shifts;
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

    #[test]
    fn owernship_test() {
        let x:u32 = 1234;
        let a = &x;
        let b = *a >> 5;

        // assert_eq!(b, *a >> 5);
        // assert_eq!(1234 as u32, x);
        assert_eq!(b, a >> 5);  // This should break because a is a ptr
    }
}