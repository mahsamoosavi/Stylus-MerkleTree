# Solidity & Stylus Merkle Benchmark

This repository benchmarks the efficiency of Merkle Trees in Solidity by generating proofs, deploying a verifier contract, and measuring gas usage. The goal is to evaluate the cost of verifying Merkle proofs on-chain.

## Overview

This repository benchmarks the efficiency of Merkle Tree verification in Solidity and Stylus by generating proofs, deploying verifier contracts, and measuring gas usage. The goal is to compare the cost of verifying Merkle proofs on-chain using Solidity (EVM) and Stylus (WASM-based Arbitrum execution).

## Prerequisites

Before proceeding, install the following dependencies:

- Node.js (for Merkle Tree generation) -- [Install Node.js](https://nodejs.org/en)
- Foundry (forge, cast) for Solidity -- [Install Foundry](https://book.getfoundry.sh/getting-started/installation)
- cargo-stylus (for Stylus contracts) -- [Install cargo-stylus](https://docs.arbitrum.io/stylus/quickstart#installing-cargo-stylus)
- jq (for parsing JSON) -- [Install jq](https://jqlang.org/download/)

Additionally, you need to run a [Nitro dev node](https://github.com/OffchainLabs/nitro-devnode/tree/main?tab=readme-ov-file) to test and deploy the contracts locally.

## Installation

1- Clone the repository:

```bash
git clone https://github.com/mahsamoosavi/solidity-stylus-merkle-benchmark.git
cd solidity-stylus-merkle-benchmark
```

2- Navigate to the `generate-proofs` directory and install dependencies:

```bash
cd generate-proofs
npm install

```

## Generating merkle proofs

1- Run the script to generate the Merkle Tree:

```bash
node generateMerkleData.js
```
2- The script will:
- Construct a Merkle Tree from addresses and token quantities.
- Hash each entry using keccak256.
- Generate a root hash and Merkle proofs.

3- The generated output will be saved as a JSON file containing:
- The Merkle root
- Leaf nodes (hashed address + quantity)
- Proofs for verification

## Compiling and deploying the smart contracts

### Solidity Deployment

1- Compile the Solidity contract using Foundry:

```bash
forge build
```
2- Deploy the contract to the dev node using `cast send`:

```bash
cast send \
  --rpc-url "http://127.0.0.1:8547" \
  --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 \
  --create "$(jq -r '.bytecode.object' out/MerkleWhitelist.sol/WhitelistVerifier.json)"
```
In the provided command:

- `--rpc-url "http://127.0.0.1:8547"`: This specifies the RPC endpoint of the Nitro dev node running locally. It allows transactions and contract interactions with the dev environment.

- `--private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659`: This is the default private key for the Nitro dev node's prefunded test account (0x3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E). It is used for signing transactions.

- `--create "$(jq -r '.bytecode.object' out/MerkleWhitelist.sol/WhitelistVerifier.json)"`: This flag deploys the contract using the extracted bytecode from the compiled Solidity contract.


## Benchmarking gas usage in Solidity & Stylus

### Solidity gas benchmark:
To measure the gas cost of verifying a Merkle proof in Solidity, execute:

```bash
cast send 0x85D9a8a4bd77b9b5559c1B7FCb8eC9635922Ed49 \
  "verifyWhitelist(bytes32[],bytes32,bytes32)(bool)" \
  "[0x6ffab96d4009ce38df68f4dc04583568617773212ffc44bef9feaece2962b766,0xb7a6405fe2217253295ac09a8724c38c054f1550bde8f10fdfe324527bb528b9]"\
  "0xa3c1274aadd82e4d12c8004c33fb244ca686dad4fcc8957fc5668588c11d9502" \
  "0x709ea5a7eb3dd2f6c2a9b2713cf3ef92de1058d6b7a47bad0c21cfac06862374" \
  --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 \
  --rpc-url http://127.0.0.1:8547
```
Note that Merkle Proof, Leaf, and Root were generated from our Merkle proof script.


Gas Used: 2513654
Gas Used for L1: 2488000