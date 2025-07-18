use crate::comb_circuits::adders::full_adder;
use crate::comb_circuits::logic_gates::not;
use crate::comb_circuits::multiplexer::four_to_one_mux;

/// Performs arithmetic operations on two 4-bit binary numbers based on select inputs
///
/// # Arguments
///
/// * `a` - First 4-bit input number as array of booleans
/// * `b` - Second 4-bit input number as array of booleans  
/// * `select` - 2-bit control input that determines the operation:
///     * 00: Addition (A + B)
///     * 01: Subtraction (A - B)
///     * 10: Increment A (A + 1)
///     * 11: Decrement A (A - 1)
/// * `carry_in` - Input carry bit
///
/// # Returns
///
/// A tuple containing 5 boolean values:
/// * carry_out: The output carry/borrow
/// * d3,d2,d1,d0: The 4-bit result from MSB to LSB
///
/// Operation	Select	y	Required carry_in
/// a + b	     00	    b	 0
/// a + b + 1	 00	    b	 1
/// a − b	     01	   ~b	 1
/// a − b − 1	 01	   ~b	 0
/// a + 1	     10	    0	 1
/// a	         10	    0	 0
/// a − 1	     01/11  ~0/1	depends (needs careful handling)

pub fn arithmetic_circuit(
    a: [bool; 4],
    b: [bool; 4],
    select: [bool; 2],
    carry_in: bool,
) -> (bool, bool, bool, bool, bool) {
    const ZERO_IN: bool = false;
    const ONE_IN: bool = true;

    // Modified 4-to-1 mux for decrement support
    let y0: bool = four_to_one_mux(select[0], select[1],
                                   b[0],           // 00: A + B
                                   not(b[0]),      // 01: A + not(B)
                                   ZERO_IN,        // 10: A + 0 (transfer)
                                   ONE_IN);        // 11: A + 1 (but we'll modify carry)

    let y1: bool = four_to_one_mux(select[0], select[1], b[1], not(b[1]), ZERO_IN, ONE_IN);
    let y2: bool = four_to_one_mux(select[0], select[1], b[2], not(b[2]), ZERO_IN, ONE_IN);
    let y3: bool = four_to_one_mux(select[0], select[1], b[3], not(b[3]), ZERO_IN, ONE_IN);

    // Special carry handling for decrement
    let initial_carry = if select == [true, true] {
        false  // For decrement: A + 11111111 + 0 = A - 1
    } else {
        carry_in
    };

    let (d0, carry_1): (bool, bool) = full_adder(a[0], y0, initial_carry);
    let (d1, carry_2): (bool, bool) = full_adder(a[1], y1, carry_1);
    let (d2, carry_3): (bool, bool) = full_adder(a[2], y2, carry_2);
    let (d3, carry_out): (bool, bool) = full_adder(a[3], y3, carry_3);

    return (carry_out, d3, d2, d1, d0);
}

