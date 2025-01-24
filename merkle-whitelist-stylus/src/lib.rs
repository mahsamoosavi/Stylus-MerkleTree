#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloy_primitives::{keccak256, B256}; // Use B256 for fixed-size 32-byte data
use stylus_sdk::prelude::*;

sol_storage! {
    #[entrypoint]
    pub struct WhitelistVerifier {}
}

#[public]
impl WhitelistVerifier {
    /// Verifies a whitelist using a Merkle proof.
    pub fn verify_whitelist(
        proof: Vec<B256>, 
        mut leaf: B256,  
        root: B256,      
    ) -> bool {
        // Preallocate a fixed buffer for hashing
        let mut buffer = [0u8; 64];

        for proof_element in proof {
            // Compare the slices directly
            if leaf < proof_element {
                buffer[..32].copy_from_slice(&leaf.0); // Convert B256 to &[u8]
                buffer[32..].copy_from_slice(&proof_element.0); // Convert B256 to &[u8]
            } else {
                buffer[..32].copy_from_slice(&proof_element.0); // Convert B256 to &[u8]
                buffer[32..].copy_from_slice(&leaf.0); // Convert B256 to &[u8]
            }

            // Update leaf with the new hash
            leaf = keccak256(&buffer).into();
        }

        // Compare the final computed hash to the root
        leaf == root
    }
}
