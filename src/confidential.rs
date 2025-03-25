use bulletproofs::{BulletproofGens, PedersenGens, ProofError, RangeProof};
use curve25519_dalek::{ristretto, Scalar};
use merlin::Transcript;

use crate::utils::generate_random_bindings;

pub struct ConfidentialTransactions {
    pc_gens: PedersenGens,
    bp_gens: BulletproofGens,
    secret_value: u64,
    blinding: Scalar,
    pub commitment: ristretto::CompressedRistretto,
    proof: Option<RangeProof>,
}
impl ConfidentialTransactions {
    pub fn new(value: u64, proof_bits: usize) -> Self {
        // Setup: Initialize generators for Pedersen commitments and Bulletproofs
        let pc_gens = PedersenGens::default();
        let bp_gens = BulletproofGens::new(proof_bits, 1);

        // Generate random blinding factor
        let random_blinding = generate_random_bindings();

        // Create commitment
        let commitment = pc_gens.commit(Scalar::from(value), random_blinding);

        ConfidentialTransactions {
            pc_gens,
            bp_gens,
            secret_value: value,
            blinding: random_blinding,
            commitment: commitment.compress(),
            proof: None,
        }
    }
    pub fn generate_proof(&mut self, proof_bits: usize) -> &RangeProof {
        let mut prover_transcript = Transcript::new(b"RangeProofExample");
        let (proof, _) = RangeProof::prove_single(
            &self.bp_gens,
            &self.pc_gens,
            &mut prover_transcript,
            self.secret_value,
            &self.blinding,
            proof_bits,
        )
        .expect("Proof creation failed");
        self.proof = Some(proof);
        self.proof.as_ref().unwrap()
    }
    pub fn verify_proof(&self, proof_bits: usize) -> Result<(), bulletproofs::ProofError> {
        if let Some(proof) = &self.proof {
            let mut verifier_transcript = Transcript::new(b"RangeProofExample");
            proof.verify_single(
                &self.bp_gens,
                &self.pc_gens,
                &mut verifier_transcript,
                &self.commitment,
                proof_bits,
            )
        } else {
            Err(ProofError::VerificationError)
        }
    }
    pub fn get_blinding(&self) -> &Scalar {
        &self.blinding
    }
}
