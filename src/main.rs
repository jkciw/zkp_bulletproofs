use bulletproof::{confidential_swap::ConfidentialSwap, ConfidentialTransactions};
fn main() {
    println!("===== Creating confidential transactions ===== \n");

    // Setup: Initialize generators for Pedersen commitments and Bulletproofs
    let proof_bits = 64;

    // Step 1: Choose a secret (Eg. Transaction amount)
    let secret_value = 42u64;
    // println!("Secret transaction amount: {}", secret_value);

    //Step 2: Create a confidential transaction
    let mut tx: ConfidentialTransactions = ConfidentialTransactions::new(secret_value, proof_bits);

    //Step 3: Create Range Proof
    tx.generate_proof(proof_bits);
    // println!("The range proof is {:?}", proof);

    //Step 4: Verify the range proof
    match tx.verify_proof(proof_bits) {
        Ok(_) => println!("The range proof is valid\n"),
        Err(e) => println!("The range proof is invalid: {:?}\n", e),
    }
    println!("===== Creating confidential Swaps  ===== \n");
    let alice_value = 600u64;
    let bob_value = 200u64;
    let expected_rate = 2u64;
    let max_allowed_diff = 1u64;
    let proof_bits = 8;
    let mut swap: ConfidentialSwap = ConfidentialSwap::new(
        alice_value,
        bob_value,
        proof_bits,
    );
    // Generating proof for the exchange rate
    let _ = swap.prove_exchange_rate(expected_rate, proof_bits, max_allowed_diff);

    //Verifying the exchange rate proof
    match swap.verify_exchange_rate(proof_bits) {
        Ok(_) => println!("The exchange rate proof is valid\n"),
        Err(e) => println!("The exchange rate proof is invalid: {:?}\n", e),
    }
}
