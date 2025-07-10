use crate::comb_circuits::logic_gates::xor;

use super::logic_gates::{and, or};

pub fn half_adder(a: bool, b: bool) -> (bool, bool) {
    let sum = xor(a, b);
    let carry = and(a, b);
    (sum, carry) // tuple return apparently
}

pub fn full_adder(a: bool, b: bool, cin: bool) -> (bool, bool) {
    // two full adder implementation
    let (sum1, carry1) = half_adder(a, b);
    let (sum2, carry2) = half_adder(sum1, cin);
    let cout = or(carry1, carry2);
    return (sum2, cout);
}
