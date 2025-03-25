use bulletproof::{confidential_swap::ConfidentialSwap, ConfidentialTransactions};
use curve25519_dalek::{ristretto, Scalar};
use std::io::{self, Write};
use std::time;

fn get_user_input<T>(prompt: &str) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    print!("{}: ", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().parse().expect("Invalid input")
}

fn display_menu() -> u32 {
    println!("\n==== Bulletproofs Demo ====");
    println!("1. Pedersen Commitments");
    println!("2. Range Proofs");
    println!("3. Interactive verification");
    println!("4. Confidential Swap");
    println!("5. Exit");
    print!("\nSelect an option: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read choice");
    choice.trim().parse().unwrap_or(0)
}

fn display_commitment(commitment: &ristretto::CompressedRistretto) -> String {
    let bytes = commitment.as_bytes();
    format!(
        "{:02x}{:02x}..{:02x}{:02x}..{:02x}{:02x}",
        bytes[0], bytes[1], bytes[10], bytes[11], bytes[30], bytes[31]
    )
}

fn display_scalar(scalar: &Scalar) -> String {
    let bytes = scalar.as_bytes();
    format!(
        "{:02x}{:02x}..{:02x}{:02x}..{:02x}{:02x}",
        bytes[0], bytes[1], bytes[10], bytes[11], bytes[30], bytes[31]
    )
}
fn pedersen_commitment_demo() {
    println!("\n===== Understanding Pedersen Commitments =====");

    let value: u64 = get_user_input("Enter a secret value to commit to");
    let proof_bits: usize = 64;

    println!("\n Generating commitment with blinding factor...");
    let tx = ConfidentialTransactions::new(value, proof_bits);

    println!("\n KNOWLEDGE SEPARATION DEMO:");
    println!("----------------------------------");
    println!("What the PROVER knows:");
    println!("  - Secret value: {}", value);
    println!("  - Blinding factor: {}", display_scalar(tx.get_blinding()));
    println!("  - Commitment hex: {}", display_commitment(&tx.commitment));
    println!("\nWhat the VERIFIER knows:");
    println!("  - Commitment: {}", display_commitment(&tx.commitment));
    println!("  - Value remains hidden!");

    println!("\n Trying with two different values:");
    let value2: u64 = get_user_input("Enter a different secret value");
    let tx2 = ConfidentialTransactions::new(value2, proof_bits);

    println!(
        "Commitment for {} = {}",
        value,
        display_commitment(&tx.commitment)
    );
    println!(
        "Commitment for {} = {}",
        value2,
        display_commitment(&tx2.commitment)
    );
    println!("\nNotice the completely different commitments");
    println!("   making it impossible to guess the value from the commitment alone!");

    println!("\nPress Enter to continue...");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}

fn range_proof_demo() {
    println!("\n===== Range Proof Analysis =====");

    let mut proof_sizes = Vec::new();
    let value: u64 = get_user_input("Enter a secret value");

    println!("\n Generating range proofs for different bit-sizes and analyzing their properties:");

    for bits in [8, 16, 32, 64].iter() {
        println!(
            "\n Generating proof that {} is in range [0, 2^{}]",
            value, bits
        );

        let start = time::Instant::now();
        let mut tx = ConfidentialTransactions::new(value, *bits);
        let proof = tx.generate_proof(*bits);
        let duration = start.elapsed();

        let proof_size = std::mem::size_of_val(proof);
        proof_sizes.push((bits, proof_size));

        println!("  - Proof generation time: {:?}", duration);
        println!("  - Proof size: approximately {} bytes", proof_size);

        println!(
            "  - Verification result: {}",
            if tx.verify_proof(*bits).is_ok() {
                "Valid"
            } else {
                "Invalid"
            }
        );
    }

    println!("\n BULLETPROOF SIZE ANALYSIS:");
    println!("----------------------------------");
    println!("Traditional range proofs grow linearly with the bit size.");
    println!("Bulletproofs grow logarithmically with the bit size!");

    println!("\nKey insight: A 64-bit Bulletproof is only slightly larger than a 32-bit one,");
    println!("   whereas traditional range proofs would double in size!");

    println!("\nPress Enter to continue...");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}

fn verification_demo() {
    println!("\n===== Interactive Verification Demo =====");

    let proof_bits: usize = get_user_input("Enter proof bit size (e.g. 16, 32, 64)");
    let secret_value: u64 = get_user_input("Enter secret value (will be hidden from verifier)");

    println!("\nPROVER PERSPECTIVE:");
    println!("----------------------------------");
    println!(
        "1. Creating commitment for value {} (this will be hidden)",
        secret_value
    );
    let mut tx = ConfidentialTransactions::new(secret_value, proof_bits);
    println!("   Commitment: {}", display_commitment(&tx.commitment));

    tx.generate_proof(proof_bits);
    println!(
        "2. Proof generated! It proves {} is in range [0, 2^{}] without revealing the value",
        secret_value, proof_bits
    );

    println!("\n VERIFIER PERSPECTIVE:");
    println!("----------------------------------");
    println!(
        "1. Received commitment: {}",
        display_commitment(&tx.commitment)
    );
    println!("2. Received proof data: [complex binary data not shown]");
    println!("3. Verifier doesn't know the secret value!");

    match tx.verify_proof(proof_bits) {
        Ok(_) => {
            println!("\nVERIFICATION SUCCEEDED!");
            println!(
                "   The verifier now knows the value is in range [0, 2^{}]",
                proof_bits
            );
            println!("   WITHOUT learning what the actual value is!");
        }
        Err(e) => {
            println!("\nVERIFICATION FAILED: {:?}", e);
        }
    }

    println!("\nTry:");
    println!("  1. Change the proof_bits to see how verification time changes");
    println!("  2. Try a negative value (should fail validation)");
    println!("  3. Try a very large value > 2^proof_bits (should fail validation)");

    println!("\nPress Enter to continue...");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}

fn confidential_swap_demo() {
    println!("\n===== Confidential Swap Demonstration =====");

    println!("\n Confidential swaps allow two parties to exchange assets");
    println!("   without revealing the exact values, while proving the exchange");
    println!("   rate is fair according to agreed-upon terms.\n");

    // Get user input for swap parameters
    let alice_asset: u64 = get_user_input("Enter Alice's asset value");
    let bob_asset: u64 = get_user_input("Enter Bob's asset value");
    let expected_rate: u64 = get_user_input("Enter expected exchange rate (Bob/Alice)");
    let proof_bits: usize = get_user_input("Enter proof bits (e.g., 16)");
    let max_exchange_diff: u64 =
        get_user_input("Enter max allowed exchange rate difference(Eg. x2, x1 ..");

    println!("\n Creating confidential swap between Alice and Bob...");
    let mut swap: ConfidentialSwap = ConfidentialSwap::new(alice_asset, bob_asset, proof_bits);

    println!("\n ALICE'S PERSPECTIVE:");
    println!("----------------------------------");
    println!("  - My asset value: {}", alice_asset);
    println!(
        "  - My commitment: {}",
        display_commitment(&swap.alice.commitment)
    );
    println!(
        "  - Bob's commitment: {}",
        display_commitment(&swap.bob.commitment)
    );
    println!(
        "  - Expected rate: {} units of Bob's asset per unit of mine",
        expected_rate
    );

    println!("\n BOB'S PERSPECTIVE:");
    println!("----------------------------------");
    println!("  - My asset value: {}", bob_asset);
    println!(
        "  - My commitment: {}",
        display_commitment(&swap.bob.commitment)
    );
    println!(
        "  - Alice's commitment: {}",
        display_commitment(&swap.alice.commitment)
    );

    println!("\n OBSERVER'S PERSPECTIVE:");
    println!("----------------------------------");
    println!(
        "  - Alice's commitment: {}",
        display_commitment(&swap.alice.commitment)
    );
    println!(
        "  - Bob's commitment: {}",
        display_commitment(&swap.bob.commitment)
    );
    println!("  - No knowledge of actual values!");

    println!("\n Generating exchange rate proof...");
    let start = time::Instant::now();
    let _ = swap.prove_exchange_rate(expected_rate, proof_bits, max_exchange_diff);
    let duration = start.elapsed();
    println!("  - Proof generation time: {:?}", duration);

    println!("\n Verifying exchange rate proof...");
    let start = time::Instant::now();
    match swap.verify_exchange_rate(proof_bits) {
        Ok(_) => println!(
            "  - VALID: The exchange rate of {} is verified!",
            expected_rate
        ),
        Err(e) => println!("  - INVALID: Exchange rate verification failed: {:?}", e),
    }
    let duration = start.elapsed();
    println!("  - Verification time: {:?}", duration);

    println!("\n KEY INSIGHTS:");
    println!("  1. Neither party reveals their exact asset value");
    println!("  2. The proof confirms the exchange rate without revealing values");
    println!("  3. This enables trust-minimized asset exchanges");
    println!("  4. Applications: DEXs, OTC trades, cross-chain swaps");

    println!("\nPress Enter to continue...");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}
fn main() {
    println!("A demonstration to help understand the working of Bulletproofs algorithm");
    loop {
        match display_menu() {
            1 => pedersen_commitment_demo(),
            2 => range_proof_demo(),
            3 => verification_demo(),
            4 => confidential_swap_demo(),
            5 => {
                println!("Exiting");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}
