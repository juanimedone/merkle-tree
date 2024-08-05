use merkle_tree::MerkleTree;

fn main() {
    // Create a new Merkle Tree with initial elements
    let elements = vec!["a", "b", "c", "d"];
    let tree = MerkleTree::new(elements).unwrap();

    // Print the root hash
    println!("Root hash: {:?}", tree.root);
}
