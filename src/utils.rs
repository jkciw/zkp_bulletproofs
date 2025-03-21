use curve25519_dalek::scalar::Scalar;
use getrandom;
pub fn generate_random_bindings() -> Scalar {
    let mut blinding_bytes = [0u8; 32];
    getrandom::fill(&mut blinding_bytes).expect("Failed to get random bytes");
    Scalar::from_bytes_mod_order(blinding_bytes)
}
