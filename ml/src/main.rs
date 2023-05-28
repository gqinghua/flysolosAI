

fn main() {
    let mut input = vec![0.1, 0.2, 0.3, 0.4, 0.5];
    let mut target = vec![0.0, 0.0, 0.0, 1.0, 1.0];

    for _ in 0..1000 {
        for i in 0..input.len() {
            let gradient = input[i] - target[i];
            input[i] += gradient * -0.1;
        }
    }
}