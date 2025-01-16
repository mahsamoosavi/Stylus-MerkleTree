// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/utils/cryptography/MerkleProof.sol";

contract MerkleWhitelist {
    bytes32 public merkleRoot;

    constructor(bytes32 _merkleRoot) {
        merkleRoot = _merkleRoot;
    }

    function verifyWhitelist(bytes32[] calldata merkleProof, address account, uint256 quantity) public view returns (bool) {
        bytes32 leaf = keccak256(abi.encodePacked(account, quantity));
        return MerkleProof.verify(merkleProof, merkleRoot, leaf);
    }
}
