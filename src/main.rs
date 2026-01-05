use sha2::{Digest, Sha256};

// The 'filing cabinet' that stores our tree levels.
// layers[0] = the bottom (leaves)
// layers[last] = the top (root)
pub struct MerkleTree {
    pub layers: Vec<Vec<String>>,
}

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

impl MerkleTree {
    pub fn new(data: Vec<String>) -> Self {
        // 1. Create the bottom layer (The Leaves/Wide part of the funnel)
        let mut first_layer = Vec::new();
        for item in data {
            first_layer.push(hash_data(&item));
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

        MerkleTree { layers }
    }

    pub fn root(&self) -> &str {
        // The root is the last layers first (and only) element
        self.layers.last().unwrap().first().unwrap()
    }
}

fn main() {
    let transactions = vec!["alice->bob:10".to_string(), "bob->charlie:5".to_string()];

    let tree = MerkleTree::new(transactions);

    println!("---------------------------------------");
    println!("Merkle Root: {}", tree.root());
    println!("Tree Depth:  {} levels", tree.layers.len());
    println!("---------------------------------------");

    //some fancy formating to visualize the tree, idk how to do it any other way yet
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_root_consistency() {
        let data = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let tree1 = MerkleTree::new(data.clone());
        let tree2 = MerkleTree::new(data);

        // The root should always be the same for the same data
        assert_eq!(tree1.root(), tree2.root());
    }

    #[test]
    fn test_odd_leaves() {
        let data = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let tree = MerkleTree::new(data);
        // 3 leaves should result in 3 levels:
        // Level 0: [H(A), H(B), H(C)]
        // Level 1: [H(AB), H(CC)]
        // Level 2: [H(ABCC)]
        assert_eq!(tree.layers.len(), 3);
    }
}
