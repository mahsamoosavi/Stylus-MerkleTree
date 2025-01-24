/* const { getAddress, AbiCoder, keccak256 } = require('ethers');

const { MerkleTree } = require('merkletreejs');

// Initialize AbiCoder
const abiCoder = new AbiCoder();

// User data
const inputs = [
  { address: '0x1230000000000000000000000000000000000000', quantity: 1 },
  { address: '0x4560000000000000000000000000000000000000', quantity: 2 },
  { address: '0x7890000000000000000000000000000000000000', quantity: 1 }
];

// Generate leaves
const leaves = inputs.map(user =>
  keccak256(abiCoder.encode(['address', 'uint256'], [user.address, user.quantity]))
);

// Create Merkle tree
const tree = new MerkleTree(leaves, keccak256, { sort: true });

console.log('Merkle Root:', tree.getHexRoot());
console.log(
  'Proofs:',
  inputs.map(user =>
    tree.getHexProof(
      keccak256(abiCoder.encode(['address', 'uint256'], [user.address, user.quantity]))
    )
  )
);





const aliceAddress = getAddress("0x1230000000000000000000000000000000000000"); // Format to checksum address
const encodedData = abiCoder.encode(['address', 'uint256'], [aliceAddress, 1]);
console.log("Encoded Data:", encodedData);
console.log("Leaf:", keccak256(encodedData).toString('hex')); */

const { keccak256, AbiCoder } = require('ethers');
const { MerkleTree } = require('merkletreejs');

// User data with quantities
const users = [
    { address: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266", quantity: 1 },
    { address: "0x70997970C51812dc3A010C7d01b50e0d17dc79C8", quantity: 2 },
    { address: "0x90F79bf6EB2c4f870365E785982E1f101E93b906", quantity: 1 }
];

// Generate leaves by hashing address and quantity together
const leaves = users.map((user) => {
    // ABI encode address and quantity, then hash
    const encodedData = new AbiCoder().encode(
        ["address", "uint256"],
        [user.address, user.quantity]
    );
    return keccak256(encodedData);
});

console.log("Leaves:", leaves);

// Create a Merkle tree
const tree = new MerkleTree(leaves, keccak256, { sortPairs: true });

console.log(tree.toString());

// Get Root of Merkle Tree
console.log(`Here is Root Hash: ${tree.getRoot().toString('hex')}`);
