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
const abiCoder = new AbiCoder();
const { MerkleTree } = require('merkletreejs');

const users = [
    { address: "0x70997970C51812dc3A010C7d01b50e0d17dc79C8", quantity: 1 },
    { address: "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC", quantity: 2 },
    { address: "0x90F79bf6EB2c4f870365E785982E1f101E93b906", quantity: 1 }
];

const leaves = users.map((user) =>
    keccak256(abiCoder.encode(['address', 'uint256'], [user.address, user.quantity]))
);

console.log("Leaves:", leaves);

// create a Merkle tree
const tree = new MerkleTree(leaves, keccak256, { sort: true });
console.log(tree.toString());