// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import "@openzeppelin/contracts/utils/cryptography/MerkleProof.sol";

contract MerkleWhitelist {
    bytes32 public root;

    constructor(bytes32 _root) {
        root = _root;
    }

    function verifyWhitelist(bytes32[] calldata merkleProof, bytes32 leaf, uint256 quantity) public view returns (bool) {
        //bytes32 leaf = keccak256(abi.encodePacked(account, quantity));
        return MerkleProof.verify(merkleProof, root, leaf);
    }
}