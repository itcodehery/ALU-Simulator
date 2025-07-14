mod comb_circuit;

use comb_circuit::logic_gates::*;
pub fn logic_circuit(a: bool, b: bool, select1: bool, select2: bool) {
    // Write Documentations
    // Four operations - and, or, xor, and not
    match (select1, select2) {
        (false,false) => and(a,b),
        (false,true) => or(a,b),
        (true,false) => xor(a,b),
        (true,true) => not(b,a),
    }
}
