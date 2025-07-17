use eframe::egui;

// Import your existing circuit modules
mod comb_circuits;


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "ALU Simulator",
        options,
        Box::new(|_cc| Ok(Box::new(ALUSim::default())),
        ))
}

#[derive(Default)]
struct ALUSim {
    operation: String,
    operation_type:String,
    result: String,
    initial_num_1: [bool; 4],
    initial_num_2: [bool; 4],
}

impl eframe::App for ALUSim  {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.operation_type.is_empty() {
                self.operation_type = "unr".parse().unwrap();
            }
            if self.operation.is_empty() && self.operation_type == "unr" {
                self.operation = "inc".parse().unwrap();
            } else if self.operation.is_empty() && self.operation_type == "bin" {
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
                });
                ui.separator();

            }
            // Bits of A and B
            ui.horizontal(|ui| {
                ui.label("Value of A (4-bit): ");
                for i in 0..4 {
                    ui.checkbox(&mut self.initial_num_1[i], format!("A{}", i));
                }

                for i in 0..4 {
                    ui.label(if self.initial_num_1[i] { "1" } else { "0" });
                }
            });
            if self.operation_type == "bin" {
                ui.horizontal(|ui| {
                    ui.label("Value of B (4-bit): ");
                    for i in 0..4 {
                        ui.checkbox(&mut self.initial_num_2[i], format!("B{}", i));
                    }
                    for i in 0..4 {
                        ui.label(if self.initial_num_2[i] { "1" } else { "0" });
                    }
                });
            }
            ui.separator();

        // Submit
            let _ = ui.button("Perform Action");

        });

    }
}