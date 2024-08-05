use merkle_tree::MerkleTree;

fn main() {
    let elements = vec!["a", "b", "c", "d"];
    let tree = MerkleTree::new(elements).unwrap();

    // Generate a proof for an element
    let proof = tree.generate_proof("a").unwrap();
    println!("Proof for 'a': {:?}", proof);
}
