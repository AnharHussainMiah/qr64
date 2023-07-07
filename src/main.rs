use rand::Rng;
use std::collections::HashMap;
use std::io::Write;

fn main() {
    println!("Rust Quantum Simulator");
    println!("Originally created by Davide Gessa (dakk)");
    println!("Ported by Coding Nexus");
    println!("Adapted by Anhar Miah");
    println!("");
    println!("Enter gate seq (x0,x1,y0,y1,z0,z1,h0,h1,cx,sw): ");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input);

    self::run_simulation(&input);
}

fn run_simulation(input: &str) {
    println!("Calculating the state vector...");
    // Initialise variables
    let mut state_vector = vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let shots = 28;
    let input_gates = input.split(',');

    for gate in input_gates {
        let _gate = gate.replace("\n", "").replace(" ", "");
        match _gate.as_str() {
            "x0" => state_vector.swap(0, 1),
            "x1" => state_vector.swap(1, 3),
            "y0" => state_vector[0] = -state_vector[0],
            "y1" => state_vector[1] = -state_vector[1],
            "z0" => state_vector[2] = -state_vector[2],
            "z1" => state_vector[1] = -state_vector[1],
            "h0" => self::apply_hadamard(&mut state_vector, 0),
            "h1" => self::apply_hadamard(&mut state_vector, 1),
            "cx" => state_vector.swap(1, 3),
            "sw" => state_vector.swap(1, 2),
            _ => println!("unknown gate '{}'", _gate),
        }
        let _ = std::io::stdout().lock().write_all(b".");
    }
    println!("");
    println!("Running {} iterations...", shots);

    let measurements = self::measure_state_vector(&mut state_vector, shots);
    self::display_results(measurements);
}

struct Measurements {
    z0: f64,
    z1: f64,
    z2: f64,
    z3: f64,
}

impl Measurements {
    pub fn as_hashmap(&self) -> HashMap<&str, f64> {
        HashMap::from([
            ("z0", self.z0),
            ("z1", self.z1),
            ("z2", self.z2),
            ("z3", self.z3),
        ])
    }
}

fn measure_state_vector(state_vector: &mut Vec<f64>, shots: i32) -> Measurements {
    let mut measurements = Measurements {
        z0: 0.0,
        z1: 0.0,
        z2: 0.0,
        z3: 0.0,
    };
    let probabilities = self::calculate_probabilities(state_vector);

    for i in 0..shots {
        let random_number: f64 = rand::thread_rng().gen_range(0.0..0.9999);
        let mut sum = 0.0;
        let mut meassured_qbit = String::new();
        for (key, probability) in probabilities.as_hashmap() {
            sum += probability;
            if random_number < sum {
                meassured_qbit = key.to_string();
                break;
            }
        }
        match meassured_qbit.as_str() {
            "z0" => measurements.z0 += 1.0,
            "z1" => measurements.z1 += 1.0,
            "z2" => measurements.z2 += 1.0,
            "z3" => measurements.z3 += 1.0,
            _ => {}
        }
    }
    measurements
}

fn calculate_probabilities(state_vector: &mut Vec<f64>) -> Measurements {
    let sq_norm = state_vector.iter().fold(0.0, |sum, amplitute| {
        sum + f64::powf((*amplitute).into(), 2.0)
    });

    if (sq_norm - 1.0).abs() > 0.00001 {
        self::normalize_state_vector(state_vector);
    }

    Measurements {
        z0: f64::powf(state_vector[0].into(), 2.0) + f64::powf(state_vector[1].into(), 2.0),
        z1: f64::powf(state_vector[2].into(), 2.0) + f64::powf(state_vector[3].into(), 2.0),
        z2: f64::powf(state_vector[4].into(), 2.0) + f64::powf(state_vector[5].into(), 2.0),
        z3: f64::powf(state_vector[6].into(), 2.0) + f64::powf(state_vector[7].into(), 2.0),
    }
}

fn display_results(measurements: Measurements) {
    println!("Results:");
    println!("00: [{}]", measurements.z0);
    println!("01: [{}]", measurements.z2);
    println!("10: [{}]", measurements.z1);
    println!("11: [{}]", measurements.z3);
}

fn normalize_state_vector(state_vector: &mut Vec<f64>) {
    let sq_norm = state_vector.iter().fold(0.0, |sum, amplitute| {
        sum + f64::powf((*amplitute).into(), 2.0)
    });

    let norm = (sq_norm as f64).sqrt();
    state_vector.into_iter().map(|&mut e| e / norm);
}

fn apply_hadamard(state_vector: &mut Vec<f64>, idx: usize) {
    let n = (2 as f64).sqrt();
    let a = (state_vector[idx] + state_vector[idx + 2]) / n;
    let b = (state_vector[idx] - state_vector[idx + 2]) / n;
    state_vector[idx] = a;
    state_vector[idx + 2] = b;
}
