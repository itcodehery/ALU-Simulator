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
        Box::new(|_cc| Ok(Box::new(AluSimulatorApp::default())),
        ))
}

#[derive(Default)]
struct AluSimulatorApp {
    // Input values
    input_a: [bool; 8],
    input_b: [bool; 8],
    operation: AluOperation,

    // Output
    result: [bool; 8],
    carry_out: bool,
    zero_flag: bool,

    // UI state
    selected_component: String,
    counter: i16
}

#[derive(Default, PartialEq)]
enum AluOperation {
    #[default]
    Add,
    Subtract,
    And,
    Or,
    Xor,
    Not,
}

impl eframe::App for AluSimulatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My App");           // <- Draw heading
            if ui.button("Click me").clicked() {  // <- Draw button, check if clicked
                self.counter += 1;          // <- Update app state
            }
            ui.label(format!("Count: {}", self.counter)); // <- Draw counter
        });

    }
}

impl AluSimulatorApp {
    fn simulate(&mut self) {
        // Convert bool arrays to your circuit format and simulate
        let a = self.bool_array_to_u8(&self.input_a);
        let b = self.bool_array_to_u8(&self.input_b);

        let result = match self.operation {
            AluOperation::Add => {
                // Use your existing adder circuit
                let (sum, carry) = self.add_8bit(a, b);
                self.carry_out = carry;
                sum
            },
            AluOperation::Subtract => {
                // Use your existing subtractor or implement
                a.wrapping_sub(b)
            },
            AluOperation::And => a & b,
            AluOperation::Or => a | b,
            AluOperation::Xor => a ^ b,
            AluOperation::Not => !a,
        };

        self.result = self.u8_to_bool_array(result);
        self.zero_flag = result == 0;
    }

    fn bool_array_to_u8(&self, arr: &[bool; 8]) -> u8 {
        let mut result = 0u8;
        for (i, &bit) in arr.iter().enumerate() {
            if bit {
                result |= 1 << i;
            }
        }
        result
    }

    fn u8_to_bool_array(&self, val: u8) -> [bool; 8] {
        let mut result = [false; 8];
        for i in 0..8 {
            result[i] = (val & (1 << i)) != 0;
        }
        result
    }

    fn add_8bit(&self, a: u8, b: u8) -> (u8, bool) {
        let sum = a as u16 + b as u16;
        ((sum & 0xFF) as u8, sum > 0xFF)
    }
}