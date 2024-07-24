use merkle_tree::MerkleTree;

#[test]
fn test_merkle_tree() {
    let elements = vec!["a", "b", "c", "d"];
    let tree = MerkleTree::new(elements.clone());

    assert_eq!(tree.leaves.len(), 4);
    assert_eq!(tree.root.len(), 64); // SHA256 hash length in hex

    let proof = tree.generate_proof("a").unwrap();
    assert!(tree.verify("a", proof));

    let proof = tree.generate_proof("b").unwrap();
    assert!(tree.verify("b", proof));

    let proof = tree.generate_proof("c").unwrap();
    assert!(tree.verify("c", proof));

    let proof = tree.generate_proof("d").unwrap();
    assert!(tree.verify("d", proof));

    assert!(tree.generate_proof("x").is_none()); 
}
