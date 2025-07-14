use crate::comb_circuits::logic_gates::{not, or, three_inp_and};

pub fn four_to_one_mux(
    select1: bool,
    select2: bool,
    inp_1: bool,
    inp_2: bool,
    inp_3: bool,
    inp_4: bool,
) -> bool {
    let not_sel_1: bool = not(select1);
    let not_sel_2: bool = not(select2);

    // A'B'I0 + A'BI1 + AB'I2  + ABI3
    let first_half: bool = or(
        three_inp_and(not_sel_1, not_sel_2, inp_1),
        three_inp_and(not_sel_1, select2, inp_2),
    );
    let second_half: bool = or(
        three_inp_and(select1, not_sel_2, inp_3),
        three_inp_and(select1, select2, inp_4),
    );

    return or(first_half, second_half);
}
