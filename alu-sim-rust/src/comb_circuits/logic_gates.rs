pub fn not(val: bool) -> bool {
    return !val;
}

pub fn and(val1: bool, val2: bool) -> bool {
    return val1 && val2;
}

pub fn or(val1: bool, val2: bool) -> bool {
    return val1 | val2;
}

pub fn xor(val1: bool, val2: bool) -> bool {
    if val1 == val2 {
        return false;
    }
    return true;
}

pub fn three_inp_and(val1: bool, val2: bool, val3: bool) -> bool {
    val1 && val2 && val3
}

pub fn two_inp_not(val1: bool, val2: bool) -> bool {
    !val1 && !val2
}
