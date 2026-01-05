# Merkle-Tree Implementation

A Rust-based implementation of a Merkle Tree data structure using SHA-256. This library provides a way to verify data integrity by hashing a collection of data into a single, unique Merkle Root.

## The Funnel Logic

Although traditionally called a tree, this project is built using a "Funnel" approach:

1. Base Layer: Raw data strings are converted into SHA-256 fingerprints.
2. Pairing: Hashes are grouped into pairs using chunks of 2.
3. Narrowing: Each pair is hashed together to create a new, single hash for the layer above. 
4. The Apex: This process repeats until the data narrows down to a single element—the Merkle Root.

If a layer has an odd number of hashes, the last hash is paired with itself. This ensures that every element is mathematically represented in the final root and maintains the binary structure.

## Features

- Binary Tree Structure: Efficiently reduces any amount of data to a 64-character hex string.
- SHA-256 Security: Utilizes the industry-standard sha2 crate.
- Odd-Node Handling: Implemented the logic to duplicate the last node when a layer is odd.
- Layer Persistence: Uses nested vectors (Vec<Vec<String>>) to store all intermediate layers.

## Technical Concepts Used

- Nested Vectors: Managed a Vec<Vec<String>> structure to act as a filing cabinet for different levels of the tree.
- Pattern Matching: Used match arms to safely differentiate between full pairs [left, right] and single nodes [left].
- Vector Slicing: Utilized .chunks(2) for clean, iterative layer building.
- Encapsulation: Private helper functions are kept internal to the implementation to expose a clean public API.
- Defensive Coding: Employed unreachable!() to handle mathematically impossible states.

## Usage

To use this in your project:

fn main() {
    let transactions = vec![
        "alice->bob:10".to_string(), 
        "bob->charlie:5".to_string()
    ];

    let tree = MerkleTree::new(transactions);
    
    println!("Merkle Root: {}", tree.root());
    println!("Tree Depth: {} levels", tree.layers.len());
}

## Testing and Verification

The implementation includes a unit test suite to verify consistency and the handling of odd-numbered data sets. These can be executed using the standard cargo test command:

cargo test

The test suite covers:
- Root Consistency: Verifying that the same input data consistently produces the same Merkle Root.
- Odd-Leaf Logic: Ensuring the tree correctly expands its depth and duplicates the final hash when an odd number of inputs is provided.
- Depth Validation: Confirming the nested vector layers match the expected mathematical depth of the tree.