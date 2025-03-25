use crate::utils::generate_random_bindings;
use bulletproofs::{BulletproofGens, PedersenGens, ProofError, RangeProof};
use curve25519_dalek::{ristretto, Scalar};
use merlin::Transcript;
use std::fmt;

#[derive(Debug)]
pub enum SwapError {
    ExchangeRateExceedThreshold {
        actual_rate: u64,
        expected_rate: u64,
        max_allowed_diff: u64,
    },
    ProofCreationError(bulletproofs::ProofError),
}
impl fmt::Display for SwapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SwapError::ExchangeRateExceedThreshold {
                actual_rate,
                expected_rate,
                max_allowed_diff,
            } => {
                write!(f, "The exchange rate difference exceeds the maximum allowed difference. Actual rate: {}, Expected rate: {}, Max allowed difference: {}", actual_rate, expected_rate, max_allowed_diff)
            }
            SwapError::ProofCreationError(e) => {
                write!(f, "Proof creation failed: {:?}", e)
            }
        }
    }
}
impl From<bulletproofs::ProofError> for SwapError {
    fn from(err: bulletproofs::ProofError) -> Self {
        SwapError::ProofCreationError(err)
    }
}
pub struct SwapParticipant {
    pub name: String,
    pub asset_value: u64,
    pub blinding_factor: Scalar,
    pub commitment: ristretto::CompressedRistretto,
}

pub struct ConfidentialSwap {
    pc_gens: PedersenGens,
    bp_gens: BulletproofGens,
    pub alice: SwapParticipant,
    pub bob: SwapParticipant,
    pub exchange_rate_commitment: Option<ristretto::CompressedRistretto>,
    pub exchange_rate_proof: Option<RangeProof>,
}

impl SwapParticipant {
    pub fn new(name: &str, asset_value: u64, pc_gens: &PedersenGens) -> Self {
        let blinding_factor = generate_random_bindings();
        let commitment = pc_gens.commit(Scalar::from(asset_value), blinding_factor);
        SwapParticipant {
            name: name.to_string(),
            asset_value,
            blinding_factor,
            commitment: commitment.compress(),
        }
    }
}
impl ConfidentialSwap {
    pub fn new(alice_asset: u64, bob_asset: u64, proof_bits: usize) -> Self {
        let pc_gens = PedersenGens::default();
        let bp_gens = BulletproofGens::new(proof_bits, 1);
        let alice = SwapParticipant::new("Alice", alice_asset, &pc_gens);
        let bob = SwapParticipant::new("Bob", bob_asset, &pc_gens);
        ConfidentialSwap {
            pc_gens,
            bp_gens,
            alice,
            bob,
            exchange_rate_commitment: None,
            exchange_rate_proof: None,
        }
    }
    // Create Range Proof for exchange rate
    pub fn prove_exchange_rate(
        &mut self,
        expected_rate: u64,
        proof_bits: usize,
        max_allowed_diff: u64,
    ) -> Result<&RangeProof, SwapError> {
        let mut prover_transcript = Transcript::new(b"RangeProofExample");
        // Calculate the exchange rate
        let actual_rate = self.bob.asset_value / self.alice.asset_value;

        // Calculate the difference between the expected and actual rate
        let rate_diff = if actual_rate > expected_rate {
            actual_rate - expected_rate
        } else {
            expected_rate - actual_rate
        };

        // Check if difference exceeds the maximum allowed difference
        if rate_diff > max_allowed_diff {
            return Err(SwapError::ExchangeRateExceedThreshold {
                actual_rate,
                expected_rate,
                max_allowed_diff,
            });
        }

        // Generate blinding factor for range proofs
        let blinding_factor = generate_random_bindings();

        // Create a commitment to the rate difference
        let commitment = self
            .pc_gens
            .commit(Scalar::from(rate_diff), blinding_factor);
        self.exchange_rate_commitment = Some(commitment.compress());
        // Create range proof for the exchange rate difference
        let (proof, _) = RangeProof::prove_single(
            &self.bp_gens,
            &self.pc_gens,
            &mut prover_transcript,
            rate_diff,
            &blinding_factor,
            proof_bits,
        )
        .expect("Proof creation failed");
        self.exchange_rate_proof = Some(proof);
        Ok(self.exchange_rate_proof.as_ref().unwrap())
    }

    // Verify the exchange rate proof
    pub fn verify_exchange_rate(&self, proof_bits: usize) -> Result<(), ProofError> {
        match (&self.exchange_rate_commitment, &self.exchange_rate_proof) {
            (Some(commitment), Some(proof)) => {
                let mut verifier_transcript = Transcript::new(b"RangeProofExample");
                proof.verify_single(
                    &self.bp_gens,
                    &self.pc_gens,
                    &mut verifier_transcript,
                    commitment,
                    proof_bits,
                )
            }
            _ => Err(ProofError::VerificationError),
        }
    }
}
