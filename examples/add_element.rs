use merkle_tree::MerkleTree;

fn main() {
    let elements = vec!["a", "b", "c", "d"];
    let mut tree = MerkleTree::new(elements);
    println!("Root hash: {:?}", tree.root.clone().unwrap());
    
    // Add a new element to the tree
    tree.add_element("e");

    // Print the new root hash
    println!("New root hash: {:?}", tree.root.unwrap());
}
