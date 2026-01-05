use sha2::{Digest, Sha256};
use std::marker::PhantomData;

// --- TRAIT DEFINITION ---

/// A contract for types that can be turned into a cryptographic fingerprint.
pub trait Hashable {
    fn to_hash(&self) -> String;
}

// Implement the contract for String so we can use our existing data.
impl Hashable for String {
    fn to_hash(&self) -> String {
        hash_data(self)
    }
}

// --- CORE DATA STRUCTURE ---

// The 'filing cabinet' that stores our tree levels.
// layers[0] = the bottom (leaves)
// layers[last] = the top (root)
#[derive(Debug)]

pub struct MerkleTree<T: Hashable> {
    pub layers: Vec<Vec<String>>,
    // Marker to link the tree to type T without storing T itself.
    _marker: PhantomData<T>,
}

// --- HELPERS ---

// Low-level helper: Turns any string into a 64-character unique fingerprint.
fn hash_data(input: &str) -> String {
    // 1. Initialize the Sha256 engine.
    // We use 'mut' (mutable) because the hasher's internal state changes as we feed it data.
    let mut hasher = Sha256::new();

    // 2. Convert the string slice (&str) into a sequence of bytes (u8).
    // Hashing algorithms operate on raw binary data, not text directly.
    hasher.update(input.as_bytes());

    // 3. "Finalize" the calculation.
    // This consumes the hasher and spits out a fixed-size byte array (32 bytes for SHA-256).
    let result = hasher.finalize();

    // 4. Transform the raw bytes into a human-readable Hexadecimal string.
    // {:x} is a format specifier that turns numbers into hex (e.g., 255 becomes "ff").
    // This is the common format you see in Bitcoin or Ethereum transaction IDs.
    format!("{:x}", result)
}

// Mid-level helper: Takes two fingerprints, glues them together, and hashes that by calling the hash_data function.
// This is how we "climb" the tree levels.
fn hash_pair(left: &str, right: &str) -> String {
    let combined = format!("{}{}", left, right);
    hash_data(&combined)
}

// --- IMPLEMENTATION ---

impl<T: Hashable> MerkleTree<T> {
    /// Creates a new Merkle Tree. Returns an Error if the data is empty.
    pub fn new(data: Vec<T>) -> Result<Self, String> {
        // Guard Clause: Prevent mathematical errors with empty inputs
        if data.is_empty() {
            return Err("Cannot create a Merkle Tree with no data.".to_string());
        }

        // 1. Create the bottom layer (The Leaves/Wide part of the funnel)
        let mut first_layer = Vec::new();
        for item in data {
            // Use the trait method here!
            first_layer.push(item.to_hash());
        }

        let mut layers = Vec::new();
        layers.push(first_layer);

        // its called a tree but we are building it the reverse way so i found it makes more sense as a "Funnel":
        // Keep creating new layers until the last layer has only 1 hash (the Root), so you start wide and go narrow
        while layers.last().unwrap().len() > 1 {
            let mut next_layer = Vec::new();

            // Look at the current top-most layer in our 'layers' vector
            let current_layer = layers.last().unwrap();

            // Process the hashes in pairs
            for chunk in current_layer.chunks(2) {
                let combined_hash = match chunk {
                    // We have two hashes == ? -> Hash them together
                    [left, right] => hash_pair(left, right),
                    // Only one hash left ? -> we hash it with itself as the last layer (every layer must be hashed)
                    [left] => hash_pair(left, left),
                    _ => unreachable!(),
                };
                next_layer.push(combined_hash);
            }

            // Add the newly created layer to our collection
            layers.push(next_layer);
        }

        Ok(MerkleTree {
            layers,
            _marker: PhantomData,
        })
    }

    pub fn root(&self) -> &str {
        // The root is the last layers first (and only) element
        self.layers.last().unwrap().first().unwrap()
    }
}

// --- MAIN EXECUTION ---

fn main() {
    let transactions = vec!["alice->bob:10".to_string(), "bob->charlie:5".to_string()];

    // Safely opening the "Result" box using a match statement
    match MerkleTree::new(transactions) {
        // Case 1: The box had a tree! We name it 'tree' and use it.
        Ok(tree) => {
            println!("---------------------------------------");
            println!("Success! Merkle Root: {}", tree.root());
            println!("Tree Depth:  {} levels", tree.layers.len());
            println!("---------------------------------------");
        }
        // Case 2: The box had an error message.
        Err(e) => {
            println!("Failed to build tree: {}", e);
        }
    }
}

// --- TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_root_consistency() {
        let data = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let tree1 = MerkleTree::new(data.clone()).unwrap();
        let tree2 = MerkleTree::new(data).unwrap();
        assert_eq!(tree1.root(), tree2.root());
    }

    #[test]
    fn test_odd_leaves() {
        let data = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let tree = MerkleTree::new(data).unwrap();
        // 3 leaves should result in 3 levels:
        // Level 0: [H(A), H(B), H(C)]
        // Level 1: [H(AB), H(CC)]
        // Level 2: [H(ABCC)]
        assert_eq!(tree.layers.len(), 3);
    }

    #[test]
    fn test_empty_data_fails() {
        let data: Vec<String> = vec![];
        let result = MerkleTree::new(data);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Cannot create a Merkle Tree with no data."
        );
    }
}
