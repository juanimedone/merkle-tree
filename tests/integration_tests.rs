use merkle_tree::{MerkleTree, Sibling};

#[test]
fn test_empty_tree() {
    let elements = vec![];
    let mut tree = MerkleTree::new(elements);

    assert_eq!(tree.leaves.len(), 0);
    assert!(tree.root.is_none());

    assert!(tree.generate_proof("a").is_none());
    assert!(!tree.verify("a", vec![Sibling::Left("test".to_string())]));

    tree.add_element("a");
    assert_eq!(tree.leaves.len(), 1);
    assert_eq!(tree.root.clone().unwrap().len(), 64); // SHA256 hash length in hex
    let proof = tree.generate_proof("a").unwrap();
    assert!(tree.verify("a", proof));

    tree.add_element("b");
    assert_eq!(tree.leaves.len(), 2);
    let proof = tree.generate_proof("b").unwrap();
    assert!(tree.verify("b", proof.clone()));

    assert!(!tree.verify("c", proof));
}

#[test]
fn test_tree_with_4_elements() {
    let elements = vec!["a", "b", "c", "d"];
    let mut tree = MerkleTree::new(elements);

    assert_eq!(tree.leaves.len(), 4);
    assert_eq!(tree.root.clone().unwrap().len(), 64);

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
    assert_eq!(tree.root.clone().unwrap().len(), 64);

    let proof = tree.generate_proof("e").unwrap();
    assert!(tree.verify("e", proof.clone()));

    assert!(!tree.verify("f", proof));
}
