use merkle_tree::MerkleTree;

fn main() {
    let elements = vec!["a", "b", "c", "d"];
    let mut tree = MerkleTree::new(elements);

    // Add a new element to the tree
    tree.add_element("e");

    // Generate and verify a proof for the new element
    let proof = tree.generate_proof("e").unwrap();
    let is_valid = tree.verify("e", proof);
    println!("Is the proof for 'e' valid? {:?}", is_valid);
}
