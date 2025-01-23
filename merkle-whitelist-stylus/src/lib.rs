#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloc::vec::Vec;
use sha3::{Digest, Keccak256};
use stylus_sdk::prelude::*;
use stylus_sdk::abi::Bytes; // Explicit import for Bytes

sol_storage! {
    #[entrypoint]
    #[doc = "The WhitelistVerifier contract provides functionality for verifying Merkle proofs."]
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
        let mut computed_hash = leaf.to_vec(); // Convert Bytes to Vec<u8>

        for p in proof {
            let mut hasher = Keccak256::new();

            // Compare slices explicitly
            if &computed_hash[..] < p.as_slice() {
                hasher.update(&computed_hash);
                hasher.update(p.as_slice());
            } else {
                hasher.update(p.as_slice());
                hasher.update(&computed_hash);
            }
            computed_hash = hasher.finalize().to_vec();
        }

        computed_hash == root.as_slice()
    }
}
