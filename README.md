# Merkle Tree

A simple implementation of a Merkle Tree in Rust. This library allows you to create a Merkle Tree, generate proofs for elements, verify elements against the Merkle Tree, and dynamically add new elements.

## Installation

To use this library in your project, add the following line to your `Cargo.toml` file under `[dependencies]`:

```toml
merkle_tree = { git = "https://github.com/juanimedone/merkle-tree.git", branch = "main" }
```

## Usage

Here's a basic example of how to use the features of the Merkle Tree library:

```rust
use merkle_tree::MerkleTree;

fn main() {
    // Create a new Merkle Tree with initial elements
    let elements = vec!["a", "b", "c", "d"];
    let mut tree = MerkleTree::new(elements);

    // Print the root hash
    println!("Root hash: {:?}", tree.root);

    // Generate a proof for an element
    let proof = tree.generate_proof("a").unwrap();
    println!("Proof for 'a': {:?}", proof);

    // Verify the proof
    let is_valid = tree.verify("a", proof);
    println!("Is the proof valid? {}", is_valid);

    // Add a new element to the tree
    tree.add_element("e");

    // Print the new root hash
    println!("New root hash: {:?}", tree.root);

    // Generate and verify a proof for the new element
    let proof = tree.generate_proof("e").unwrap();
    let is_valid = tree.verify("e", proof);
    println!("Is the proof for 'e' valid? {}", is_valid);
}
```
