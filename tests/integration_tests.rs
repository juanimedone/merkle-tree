use merkle_tree::MerkleTree;

#[test]
fn test_tree_with_4_elements() {
    let elements = vec!["a", "b", "c", "d"];
    let mut tree = MerkleTree::new(elements.clone());

    assert_eq!(tree.leaves.len(), 4);
    assert_eq!(tree.root.len(), 64); // SHA256 hash length in hex

    let mut proof = tree.generate_proof("a").unwrap();
    assert!(tree.verify("a", proof.clone()));
    proof.pop();
    assert!(!tree.verify("a", proof));

    let proof = tree.generate_proof("b").unwrap();
    assert!(tree.verify("b", proof));

    let proof = tree.generate_proof("c").unwrap();
    assert!(tree.verify("c", proof));

    let proof = tree.generate_proof("d").unwrap();
    assert!(tree.verify("d", proof));

    assert!(tree.generate_proof("x").is_none()); 

    tree.add_element("e");
    assert_eq!(tree.leaves.len(), 5);
    assert_eq!(tree.root.len(), 64);

    let proof = tree.generate_proof("e").unwrap();
    assert!(tree.verify("e", proof.clone()));

    assert!(!tree.verify("f", proof));
}
