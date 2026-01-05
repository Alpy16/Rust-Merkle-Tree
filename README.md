# Merkle-Tree Implementation

A generic Rust-based implementation of a Merkle Tree data structure using SHA-256. This library provides a way to verify data integrity by hashing a collection of any "Hashable" data into a single, unique Merkle Root.

## The Funnel Logic

Although traditionally called a tree, this project is built using a "Funnel" approach:

1. Base Layer: Data is converted into SHA-256 fingerprints using a custom Trait.
2. Pairing: Hashes are grouped into pairs using chunks of 2.
3. Narrowing: Each pair is hashed together to create a new, single hash for the layer above. 
4. The Apex: This process repeats until the data narrows down to a single element—the Merkle Root.

If a layer has an odd number of hashes, the last hash is paired with itself. This ensures that every element is mathematically represented in the final root and maintains the binary structure.

## Features

- Generic Abstraction: Works with any data type (String, Transactions, etc.) that implements the Hashable trait.
- SHA-256 Security: Utilizes the industry-standard sha2 crate.
- Safe Error Handling: Returns a Result type to prevent crashes on empty inputs.
- Odd-Node Handling: Automatically duplicates the last node when a layer is odd to maintain tree balance.
- Layer Persistence: Uses nested vectors (Vec<Vec<String>>) to act as a filing cabinet for all intermediate layers.

## Technical Concepts Used

- Traits: Defined a Hashable contract to allow the tree to handle diverse data types.
- Generics & Trait Bounds: Used <T: Hashable> to make the tree a reusable container.
- PhantomData: Employed std::marker::PhantomData to solve the 'Unused Type Parameter' error, allowing the tree to be logically tied to a specific data type for type-safety.
- Result & Match: Implemented a guard clause to return an Err for empty vectors instead of panicking, requiring the user to safely handle the "Result box."
- Pattern Matching: Used match arms within .chunks(2) to safely differentiate between full pairs and single nodes.

## Usage

To use this in your project:

fn main() {
    let transactions = vec![
        "alice->bob:10".to_string(), 
        "bob->charlie:5".to_string()
    ];

    match MerkleTree::new(transactions) {
        Ok(tree) => {
            println!("Merkle Root: {}", tree.root());
            println!("Tree Depth: {} levels", tree.layers.len());
        },
        Err(e) => println!("Error: {}", e),
    }
}

## Testing and Verification

The implementation includes a unit test suite to verify consistency and safety. These can be executed using the standard cargo test command:

cargo test

The test suite covers:
- Root Consistency: Verifying that the same input data consistently produces the same Merkle Root.
- Odd-Leaf Logic: Ensuring the tree correctly expands its depth when an odd number of inputs is provided.
- Safe Failure: Testing that an empty input vector returns a proper Err rather than a crash.