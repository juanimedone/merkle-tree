# Merkle Tree

A simple implementation of a Merkle Tree in Rust. This library allows you to create a Merkle Tree, generate proofs for elements, verify elements against the Merkle Tree, and dynamically add new elements.

## Installation

To use this library in your project, add the following line to your `Cargo.toml` file under `[dependencies]`:

```toml
merkle_tree = { git = "https://github.com/juanimedone/merkle-tree.git", branch = "main" }
```

## Usage

### Creating a new Merkle Tree

To create a new Merkle Tree with initial elements, use the `new` method. This initializes the tree with the given elements and computes the root hash.

```rust
use merkle_tree::MerkleTree;

fn main() {
    // Create a new Merkle Tree with initial elements
    let elements = vec!["a", "b", "c", "d"];
    let tree = MerkleTree::new(elements).unwrap();

    // Print the root hash
    println!("Root hash: {:?}", tree.root);
}
```

### Generating a proof for an element

You can generate a proof for an element using the `generate_proof` method. This proof can be later used to verify that the element is part of the Merkle Tree.

```rust
use merkle_tree::MerkleTree;

fn main() {
    let elements = vec!["a", "b", "c", "d"];
    let tree = MerkleTree::new(elements).unwrap();

    // Generate a proof for an element
    let proof = tree.generate_proof("a").unwrap();
    println!("Proof for 'a': {:?}", proof);
}
```

### Verifying a proof

To verify that a proof is valid for a given element, use the `verify` method. This checks whether the element is part of the Merkle Tree based on the provided proof.

```rust
use merkle_tree::MerkleTree;

fn main() {
    let elements = vec!["a", "b", "c", "d"];
    let tree = MerkleTree::new(elements).unwrap();
    let proof = tree.generate_proof("a").unwrap();

    // Verify the proof
    let is_valid = tree.verify("a", proof);
    println!("Is the proof valid? {:?}", is_valid);
}
```

### Adding a new element

You can dynamically add new elements to the Merkle Tree using the `add_element` method. This will recalculate the root hash based on the updated set of leaves.

```rust
use merkle_tree::MerkleTree;

fn main() {
    let elements = vec!["a", "b", "c", "d"];
    let mut tree = MerkleTree::new(elements).unwrap();
    println!("Root hash: {:?}", tree.root.clone());

    // Add a new element to the tree
    tree.add_element("e");

    // Print the new root hash
    println!("New root hash: {:?}", tree.root);
}
```

### Generating and verifying proofs for new elements

After adding a new element, you can generate and verify a proof for it using the same methods as before.

```rust
use merkle_tree::MerkleTree;

fn main() {
    let elements = vec!["a", "b", "c", "d"];
    let mut tree = MerkleTree::new(elements).unwrap();

    // Add a new element to the tree
    tree.add_element("e");

    // Generate and verify a proof for the new element
    let proof = tree.generate_proof("e").unwrap();
    let is_valid = tree.verify("e", proof);
    println!("Is the proof for 'e' valid? {:?}", is_valid);
}
```
