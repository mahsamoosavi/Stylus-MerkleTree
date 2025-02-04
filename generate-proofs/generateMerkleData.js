const { keccak256, AbiCoder } = require('ethers'); 
const { MerkleTree } = require('merkletreejs'); 
const fs = require('fs'); 

// Users list with address and quantity
const users = [
    { address: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266", quantity: 1 },
    { address: "0x70997970C51812dc3A010C7d01b50e0d17dc79C8", quantity: 2 },
    { address: "0x90F79bf6EB2c4f870365E785982E1f101E93b906", quantity: 1 }
];

// Generate leaves (hash of encoded address + quantity)
const leaves = users.map((user) => {
    const encodedData = new AbiCoder().encode(["address", "uint256"], [user.address, user.quantity]);
    return keccak256(encodedData);
});

// Print Leaves
console.log("\n🔹 **Leaves**:");
const leafData = users.map((user, index) => ({
    address: user.address,
    quantity: user.quantity,
    leaf: leaves[index]
}));

leafData.forEach((leaf, index) => {
    console.log(`${index + 1}. ${leaf.address} → ${leaf.leaf}`);
});

// Create Merkle Tree
const tree = new MerkleTree(leaves, keccak256, { sortPairs: true });

// Print Merkle Tree Structure
console.log("\n🔹 **Merkle Tree:**\n", tree.toString());

// Get Merkle Root
const root = tree.getHexRoot();
console.log(`\n🌳 **Merkle Root:** ${root}\n`);

// Generate Proofs for each user
const userProofs = users.map((user) => {
    const encodedData = new AbiCoder().encode(["address", "uint256"], [user.address, user.quantity]);
    const proof = tree.getHexProof(keccak256(encodedData));
    console.log(`✅ Proof for ${user.address}:`, proof);
    return { address: user.address, quantity: user.quantity, proof };
});

// Save to JSON file
const output = { 
    root, 
    leaves: leafData,   // ✅ Includes leaves in the JSON file
    data: userProofs    // ✅ Includes user proofs
};
fs.writeFileSync('merkleData.json', JSON.stringify(output, null, 2));

console.log('\n📄 **Merkle data saved to merkleData.json**');
