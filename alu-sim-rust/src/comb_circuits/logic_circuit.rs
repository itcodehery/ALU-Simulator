use super::logic_gates::*;

pub fn logic_circuit(a: [bool; 4], b: [bool; 4], operation: &str) -> [bool; 4] {
    // Write Documentations
    // Four operations - and, or, xor, and not
    let mut result: [bool; 4] = [false; 4];

    match operation {
        "and" => {
            for i in 0..4 {
                result[i] = bitwise_logic(a[i], b[i], false, false);
            }
        }
        "or" => {
            for i in 0..4 {
                result[i] = bitwise_logic(a[i], b[i], false, true);
            }
        }
        "xor" => {
            for i in 0..4 {
                result[i] = bitwise_logic(a[i], b[i], true, false);
            }
        }
        "not" => {
            for i in 0..4 {
                result[i] = bitwise_logic(a[i], b[i], true, true);
            }
        }
        _ => {
            println!("Panic: Invalid Operation!");
        }
    }
    return result;
}

pub fn bitwise_logic(a: bool, b: bool, select1: bool, select2: bool) -> bool {
    // Write Documentations
    // Four operations - and, or, xor, and not
    match (select1, select2) {
        (false, false) => and(a, b),
        (false, true) => or(a, b),
        (true, false) => xor(a, b),
        (true, true) => two_inp_not(b, a),
    }
}
