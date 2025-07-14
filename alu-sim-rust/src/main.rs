mod comb_circuits {
    pub mod adders;
    pub mod arithmetic_circuit;
    pub mod logic_gates;
    pub mod multiplexer;
}

use std::thread::sleep;
use std::time::Duration;

use comb_circuits::adders::{full_adder, half_adder};
use comb_circuits::logic_gates::{and, not, or, xor};

use comb_circuits::arithmetic_circuit::arithmetic_circuit;

fn initialize_alu() {
    println!("------------------------");
    println!("Arithmetic and Logic Unit");
    println!("------------------------");
    println!("Initializing simulation...");
    // Thread functions
    let def_dur: Duration = Duration::new(2, 0);
    sleep(def_dur);
}
fn main() {
    initialize_alu();
    show_menu();
}

// This should eventually ask the user these options:
// 1. Unary Functions
//      a. Complement
// 2. Binary Functions
fn show_menu() {
    println!("Select operation:");
    println!("1. Half Adder");
    println!("2. Full Adder");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => half_adder_call(),
        "2" => full_adder_call(),
        _ => println!("Invalid choice"),
    }
}

fn half_adder_call() {
    println!("--------------------\n");
    println!("Half Adder:");
    println!("Enter two Binary values: ");
    println!("Input 1: ");
    let mut input1: String = String::new();
    std::io::stdin().read_line(&mut input1).unwrap();
    println!("Input 2: ");
    let mut input2: String = String::new();
    std::io::stdin().read_line(&mut input2).unwrap();
    let input1: bool = input1.trim().parse().unwrap();
    let input2: bool = input2.trim().parse().unwrap();
    let (sum, carry) = half_adder(input1, input2);
    println!("Sum: {}", sum);
    println!("Carry: {}", carry);
    println!("--------------------\n");
}

fn full_adder_call() {
    println!("--------------------\n");
    println!("Full Adder:");
    println!("Enter three 'true' or 'false' values: ");
    println!("Input 1: ");
    let mut input1: String = String::new();
    std::io::stdin().read_line(&mut input1).unwrap();
    println!("Input 2: ");
    let mut input2: String = String::new();
    std::io::stdin().read_line(&mut input2).unwrap();
    println!("Input 3 (Carry In): ");
    let mut input3: String = String::new();
    std::io::stdin().read_line(&mut input3).unwrap();

    let input1: bool = input1.trim().parse().unwrap();
    let input2: bool = input2.trim().parse().unwrap();
    let input3: bool = input3.trim().parse().unwrap();

    let (sum, carry) = full_adder(input1, input2, input3);
    println!("Sum: {}", sum);
    println!("Carry: {}", carry);
    println!("--------------------\n");
}
