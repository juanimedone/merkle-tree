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
