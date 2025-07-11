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
    if (val1 == val2) {
        return false;
    }
    return true;
}

pub fn nand(val1: bool, val2: bool) -> bool {
    return !(and(val1, val2));
}

pub fn nor(val1: bool, val2: bool) -> bool {
    return !(or(val1, val2));
}

pub fn three_inp_and(val1: bool, val2: bool, val3: bool) -> bool {
    val1 && val2 && val3
}
