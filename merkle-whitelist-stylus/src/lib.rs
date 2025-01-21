
//!
//! Stylus Whitelist Verification Example
//!
//! This contract verifies a whitelist using a Merkle proof.
//!

#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloc::vec::Vec;
use sha3::{Digest, Keccak256};
use stylus_sdk::prelude::*;

sol_storage! {
    #[entrypoint]
    #[doc = "The WhitelistVerifier contract provides functionality for verifying Merkle proofs."]
    pub struct WhitelistVerifier {}
}

#[public]
impl WhitelistVerifier {
    /// Verifies a whitelist using a Merkle proof.
    pub fn verify_whitelist(
        proof: Vec<Vec<u8>>,
        leaf: Vec<u8>,
        root: Vec<u8>,
    ) -> bool {
        let mut computed_hash = leaf;

        for p in proof {
            let mut hasher = Keccak256::new();
            if computed_hash < p {
                hasher.update(&computed_hash);
                hasher.update(&p);
            } else {
                hasher.update(&p);
                hasher.update(&computed_hash);
            }
            computed_hash = hasher.finalize().to_vec();
        }

        computed_hash == root
    }
}

