#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    
    let weights = sp1_zkvm::io::read::<Vec<f64>>();
    let bias = sp1_zkvm::io::read::<f64>();
    let input = sp1_zkvm::io::read::<Vec<f64>>();
    let output = sp1_zkvm::io::read::<f64>();


    let pred_op = mul_weights(weights, input.clone()) + bias;
    let is_true = format!("{:.2}", pred_op).parse::<f64>().unwrap()==format!("{:.2}", output).parse::<f64>().unwrap();
    sp1_zkvm::io::write(&input);
    sp1_zkvm::io::write(&output);
    sp1_zkvm::io::write(&is_true);


}

fn mul_weights(w: Vec<f64>, inp: Vec<f64>) -> f64{
    let mut x : f64 = 0.0;
    for i in 0..w.len(){
        x += w[i]*inp[i];
    }
    x
}


