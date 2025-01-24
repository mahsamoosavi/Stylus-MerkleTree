// // SPDX-License-Identifier: MIT
// pragma solidity ^0.8.26;

// import "forge-std/Test.sol";
// import "../src/MerkleWhitelist.sol";

// contract MerkleWhitelistTest is Test {
//     MerkleWhitelist whitelist;
//     bytes32 root;

//     function setUp() public {
//         // Initialize the MerkleWhitelist contract with the root generated from the script
//         root = 0x93170468739b9113a8753be156a16b5652e30508503f5a63cf5e1dbd6fd93296

// ; // Example root
//         whitelist = new MerkleWhitelist(root);
//     }

//     function testVerifyValidProofForAlice() public {
//         // Generate proof for Alice dynamically
//         bytes32[] memory proof=new bytes32[](2); // Replace with actual dynamic proof generation function
//         proof[0] = 0x18262819259075f0859ea45625ec1ee09b842b15d0fe804df64c2b7fa1b7284a; // Sibling (Carol's hash)
//         proof[1] = 0xb603ca691aa0d0ecd0b23afbf705f86ee2309d5b7538017d6355987c2ea8ca9c; // Parent's sibling (Bob's hash)

//        // Leaf (Alice's hash)
//         bytes32 leaf = 0x956c29270743256716ac81b12ec6b311915938037d99a40a5651ca317b41cfa6;

//         // Verify the proof for Alice
//         assertTrue(whitelist.verifyWhitelist(proof, leaf, 1), "Valid proof for Alice should succeed");
//     }

//     function testVerifyValidProofForBob() public {
//         // Generate proof for Alice dynamically
//         bytes32[] memory proof=new bytes32[](2); // Replace with actual dynamic proof generation function
//         proof[0] = 0x956c29270743256716ac81b12ec6b311915938037d99a40a5651ca317b41cfa6; // Sibling (Carol's hash)
//         proof[1] = 0xb603ca691aa0d0ecd0b23afbf705f86ee2309d5b7538017d6355987c2ea8ca9c; // Parent's sibling (Bob's hash)

//        // Leaf (Alice's hash)
//         bytes32 leaf = 0x18262819259075f0859ea45625ec1ee09b842b15d0fe804df64c2b7fa1b7284a;

//         // Verify the proof for Alice
//         assertTrue(whitelist.verifyWhitelist(proof, leaf, 2), "Valid proof for Alice should succeed");
//     }
//     function testVerifyValidProofForCarol() public {
//         // Generate proof for Alice dynamically
//         bytes32[] memory proof=new bytes32[](1); // Replace with actual dynamic proof generation function
//         proof[0] = 0x2438c937c05bef45c37fa914b9e004649857a3713e5490830c1abff90cbf36bd; // Sibling (Carol's hash)

//        // Leaf (Alice's hash)
//         bytes32 leaf = 0xb603ca691aa0d0ecd0b23afbf705f86ee2309d5b7538017d6355987c2ea8ca9c;

//         // Verify the proof for Alice
//         assertTrue(whitelist.verifyWhitelist(proof, leaf, 1), "Valid proof for Alice should succeed");
//     }

//     // function testVerifyValidProofForBob() public {
//     //     // Generate proof for Bob dynamically
//     //     bytes32[] memory proof = getProofForBob(); // Replace with actual dynamic proof generation function

//     //     address bob = 0xbd19ff506d92b45639170e62f1a12073921a4358c3ccd05d4584519f78d65103;
//     //     uint256 quantity = 2;

//     //     // Verify the proof for Bob
//     //     assertTrue(whitelist.verifyWhitelist(proof, bob, quantity), "Valid proof for Bob should succeed");
//     // }

//     // function testVerifyValidProofForCarol() public {
//     //     // Generate proof for Carol dynamically
//     //     bytes32[] memory proof = getProofForCarol(); // Replace with actual dynamic proof generation function

//     //     address carol = 0xb7a6405fe2217253295ac09a8724c38c054f1550bde8f10fdfe324527bb528b9;
//     //     uint256 quantity = 1;

//     //     // Verify the proof for Carol
//     //     assertTrue(whitelist.verifyWhitelist(proof, carol, quantity), "Valid proof for Carol should succeed");
//     // }

    
// }
