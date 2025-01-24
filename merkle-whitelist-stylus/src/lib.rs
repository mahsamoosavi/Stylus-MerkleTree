#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloc::vec::Vec;
use alloy_primitives::keccak256; // Import keccak256 for hashing and B256 for fixed-size data
use stylus_sdk::prelude::*;
use stylus_sdk::abi::Bytes;

sol_storage! {
    #[entrypoint]
    pub struct WhitelistVerifier {}
}

#[public]
impl WhitelistVerifier {
    /// Verifies a whitelist using a Merkle proof.
    pub fn verify_whitelist(
        proof: Vec<Bytes>, // The proof as a vector of Bytes
        leaf: Bytes,       // The leaf as Bytes
        root: Bytes,       // The root as Bytes
    ) -> bool {
        // Start with the leaf as the initial computed hash
        let mut computed_hash = leaf.to_vec();

        // Iterate through the proof elements
        for p in proof {
            let proof_element = p.as_slice(); // Use &[u8] for the current proof element

            // Compare as slices
            computed_hash = if computed_hash.as_slice() < proof_element {
                keccak256(&[computed_hash.as_slice(), proof_element].concat()).to_vec()
            } else {
                keccak256(&[proof_element, computed_hash.as_slice()].concat()).to_vec()
            };
        }

        // Compare the final computed hash to the root
        computed_hash.as_slice() == root.as_slice()
    }
}
