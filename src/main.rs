extern crate floatpt;

fn main() {
    /* Okay, this is a software implementation of IEEE 32 bit (short) Floating Point
       FP is a 32 bit number with the HO bit being a sign bit, followed by 8-bit
       exponent.  The final 23 bits are the mantisa.  Note, that there is an implied
       24th bit to the mantisa, it is always one.  i.e. 1.000e1

       Also, remeber that the 2^0 is represented by 127 (excess 127) or 0x7f

       The final thing to remember.  There are special numbers when the exponent is
       all 0 or all 1.  We will cover this later when dealing with the numbers
    */
    println!("There is no error");
}