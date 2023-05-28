

use num::{Float, One, Zero};
use rand::{thread_rng, Rng};

const INPUT_SIZE: usize = 4;
const HIDDEN_SIZE: usize = 3;
const OUTPUT_SIZE: usize = 1;

struct NeuralNetwork {
    weights_input_hidden: Vec<f32>,
    weights_hidden_output: Vec<f32>,
}

impl NeuralNetwork {
    fn new() -> NeuralNetwork {
        NeuralNetwork {
            weights_input_hidden: vec![
                thread_rng().gen::<f32>() * 0.1,
                thread_rng().gen::<f32>() * 0.1,
                thread_rng().gen::<f32>() * 0.1,
                thread_rng().gen::<f32>() * 0.1,
            ],
            weights_hidden_output: vec![
                thread_rng().gen::<f32>() * 0.1,
                thread_rng().gen::<f32>() * 0.1,
                thread_rng().gen::<f32>() * 0.1,
            ],
        }
    }
}

impl NeuralNetwork {
    fn train(&mut self, input: &[f32], target: &[f32]) {
        let mut output = [0.0; OUTPUT_SIZE];
        let mut hidden = [0.0; HIDDEN_SIZE];
        let mut delta_hidden = [0.0; HIDDEN_SIZE];
        let mut delta_output = [0.0; OUTPUT_SIZE];
        let mut learning_rate = 0.1;
        let mut momentum = 0.0;
        let mut momentum2 = 0.0;
        let mut epochs = 10;
        let mut training_epochs = 10;
        let mut input_size = INPUT_SIZE;
        let mut hidden_size = HIDDEN_SIZE;
        let mut output_size = OUTPUT_SIZE;
        let mut rng = thread_rng();
        let mut weights_input_hidden: Vec<f32> = vec![];
        let mut weights_hidden_output: Vec<f32> = vec![];
    }
}

fn main() {
    println!("Hello, world!");
}