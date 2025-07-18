use eframe::egui;
use crate::comb_circuits::{arithmetic_circuit::arithmetic_circuit, logic_circuit::logic_circuit};

// Import your existing circuit modules
mod comb_circuits;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {

        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "ALU Simulator",
        options,
        Box::new(|_cc| Ok(Box::new(ALUSim::default()))),
    )
}

#[derive(Default)]
struct ALUSim {
    operation: String,
    operation_type: String,
    result: [bool; 5],
    initial_num_1: [bool; 4],
    initial_num_2: [bool; 4],
    show_dialog: bool,
}

fn display_bool_as_int(a: bool) -> i32 {
    if a {
        1
    } else {
        0
    }
}

impl ALUSim {
    fn show_confirmation_dialog(&mut self, ctx: &egui::Context) {
        egui::Window::new("Confirm Action")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.label("No Operation Selected!");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if ui.button("Okay").clicked() {
                        self.show_dialog = false;
                    }

                });
            });
    }
}
impl eframe::App for ALUSim {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.operation_type.is_empty() {
                self.operation_type = "unr".parse().unwrap();
            }
            if self.operation.is_empty() && self.operation_type == "unr" {
                self.operation = "inc".parse().unwrap();
            } else if self.operation_type == "bin" && self.operation_type.is_empty() {
                self.operation = "add".parse().unwrap();
            }

            // Heading
            ui.heading("ALU Simulator");
            ui.separator();

            // Operation Type
            ui.horizontal(|ui| {
                ui.label("Operation Type");
                ui.radio_value(&mut self.operation_type, "unr".parse().unwrap(), "Unary");
                ui.radio_value(&mut self.operation_type, "bin".parse().unwrap(), "Binary");
            });

            // Operation to be Performed
            if self.operation_type == "unr" {
                ui.horizontal(|ui| {
                    ui.label("Operation: ");
                    ui.radio_value(&mut self.operation, "inc".parse().unwrap(), "Increment");
                    ui.radio_value(&mut self.operation, "dec".parse().unwrap(), "Decrement");
                });
                ui.separator();
            }

            if self.operation_type == "bin" {
                ui.horizontal(|ui| {
                    ui.label("Operation: ");
                    ui.radio_value(&mut self.operation, "add".parse().unwrap(), "Addition");
                    ui.radio_value(&mut self.operation, "sub".parse().unwrap(), "Subtraction");
                    ui.radio_value(&mut self.operation, "and".parse().unwrap(), "Bitwise AND");
                    ui.radio_value(&mut self.operation, "or".parse().unwrap(), "Bitwise OR");
                    ui.radio_value(&mut self.operation, "xor".parse().unwrap(), "Bitwise XOR");
                });
                ui.separator();
            }
            // Bits of A and B

            ui.label("Value of A (4-bit): ");
            ui.horizontal(|ui| {
                for i in (0..4).rev() {
                    ui.checkbox(&mut self.initial_num_1[i], format!("A{}", i));
                }

            });


            if self.operation_type == "bin" {

                ui.label("Value of B (4-bit): ");
                ui.horizontal(|ui| {
                    for i in (0..4).rev() {
                        ui.checkbox(&mut self.initial_num_2[i], format!("B{}", i));
                    }

                });

            }
            ui.separator();

            ui.label("A:");
            ui.horizontal(|ui| {
                for i in (0..4).rev() {
                    ui.label(if self.initial_num_1[i] { "1" } else { "0" });
                }
            });
            ui.label("B:");
            ui.horizontal(|ui| {
                for i in (0..4).rev() {
                    ui.label(if self.initial_num_2[i] { "1" } else { "0" });
                }
            });
            // Submit
            ui.horizontal(|ui| {
                let response = ui.button("Perform Action");
                if response.clicked() {
                    self.result = [false; 5];
                    if self.operation_type == "bin" {
                        if self.operation != "add" && self.operation != "sub" {
                            let temp_array =
                                logic_circuit(self.initial_num_1, self.initial_num_2, &self.operation);
                            for i in 1..=4 {
                                self.result[i] = temp_array[4 - i];
                            }
                        }
                        if self.operation == "add" {
                            self.result = <[bool; 5]>::from(arithmetic_circuit(
                                self.initial_num_1,
                                self.initial_num_2,
                                [false, false],
                                false,
                            ));
                        }
                        if self.operation == "sub" {
                            self.result = <[bool; 5]>::from(arithmetic_circuit(
                                self.initial_num_1,
                                self.initial_num_2,
                                [false, true],
                                true,
                            ));

                        }

                    }
                    if self.operation_type == "unr" {
                        if self.operation == "inc" {
                            self.result = <[bool; 5]>::from(arithmetic_circuit(self.initial_num_1, [true, false, false, false], [true, false], true));
                        }
                        if self.operation == "dec" {
                            self.result = <[bool; 5]>::from(arithmetic_circuit(self.initial_num_1, [true, false, false, false], [true, true], false));
                        }
                    }
                }
                let reset = ui.button("Reset");
                if reset.clicked() {
                    self.result = [false; 5];
                    self.initial_num_1 = [false; 4];
                    self.initial_num_2 = [false; 4];
                }
            });

            ui.separator();

            // Display results
            if !self.result.is_empty() {
                ui.horizontal(|ui| {
                    ui.label("Result: ");
                    if self.operation == "sub" || self.operation == "dec" {
                        self.result[0] = false;
                    }
                    for i in 0..5 {

                        ui.label(format!("{}", display_bool_as_int(self.result[i])));
                    }

                });
            }
        });
        if self.show_dialog {
            self.show_confirmation_dialog(ctx);
        }
    }
}

