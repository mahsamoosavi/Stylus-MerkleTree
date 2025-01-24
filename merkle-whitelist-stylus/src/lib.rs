#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloc::vec::Vec;
use sha3::{Digest, Keccak256};
use stylus_sdk::prelude::*;
use stylus_sdk::abi::Bytes; // Explicit import for Bytes

sol_storage! {
    #[entrypoint]
    pub struct WhitelistVerifier {}
}

#[public]
impl WhitelistVerifier {
    /// Verifies a whitelist using a Merkle proof.
    pub fn verify_whitelist(
        proof: Vec<Bytes>, // Use Bytes for each proof entry
        leaf: Bytes,       // Use Bytes for the leaf
        root: Bytes,       // Use Bytes for the Merkle root
    ) -> bool {
        // Preallocate memory to avoid dynamic memory overhead
        let mut computed_hash = Vec::with_capacity(leaf.len());
        computed_hash.extend_from_slice(&leaf);

        // Reuse the same Keccak256 hasher instance
        let mut hasher = Keccak256::new();

        for p in proof {
            hasher.reset(); // Reset the hasher state for the new hash
            if &computed_hash[..] < p.as_slice() {
                hasher.update(&computed_hash);
                hasher.update(p.as_slice());
            } else {
                hasher.update(p.as_slice());
                hasher.update(&computed_hash);
            }
            computed_hash = hasher.finalize_reset().to_vec(); // Use `finalize_reset` to reuse the hasher
        }

        // Compare the computed hash with the provided root
        computed_hash == root.as_slice()
    }
}
