//! A simple script to generate and verify the proof of a given program.

use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Generate proof.
    let mut stdin = SP1Stdin::new();
    let weights = vec![25.45380431, 38.79085534,  0.22021946, 61.48950341];
    let bias = -1035.0895982778834;
    let input = vec![33.4995061 , 11.94659088, 36.48632507,  3.93786264];
    let output = 531.19097047;
    stdin.write(&weights);
    stdin.write(&bias);
    stdin.write(&input);
    stdin.write(&output);
    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    // Read output.
    let inp = proof.stdout.read::<Vec<f64>>();
    let op = proof.stdout.read::<f64>();
    let res = proof.stdout.read::<bool>();
    println!("Inputs: {:?}", inp);
    println!("Ouput: {}", op);
    println!("Result: {}", res);

    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("succesfully generated and verified proof for the program!");
}
