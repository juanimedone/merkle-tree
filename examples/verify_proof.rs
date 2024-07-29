use merkle_tree::MerkleTree;

fn main() {
    let elements = vec!["a", "b", "c", "d"];
    let tree = MerkleTree::new(elements);
    let proof = tree.generate_proof("a").unwrap();

    // Verify the proof
    let is_valid = tree.verify("a", proof);
    println!("Is the proof valid? {:?}", is_valid);
}
