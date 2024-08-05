use merkle_tree::MerkleTree;

#[test]
fn test_create_empty_tree() {
    let elements = vec![];
    let tree = MerkleTree::new(elements);

    assert!(tree.is_err())
}

#[test]
fn test_tree_with_3_elements() {
    let elements = vec!["a", "b", "c"];
    let mut tree = MerkleTree::new(elements).unwrap();

    assert_eq!(tree.leaves.len(), 3);
    assert_eq!(tree.root.clone().len(), 64); // SHA256 hash length in hex

    let mut proof = tree.generate_proof("a").unwrap();
    assert!(tree.verify("a", proof.clone()));
    proof.pop();
    assert!(!tree.verify("a", proof));

    let proof = tree.generate_proof("b").unwrap();
    assert!(tree.verify("b", proof));

    let proof = tree.generate_proof("c").unwrap();
    assert!(tree.verify("c", proof));

    assert!(tree.generate_proof("x").is_none());

    tree.add_element("d");
    assert_eq!(tree.leaves.len(), 4);
    assert_eq!(tree.root.clone().len(), 64);

    let proof = tree.generate_proof("d").unwrap();
    assert!(tree.verify("d", proof.clone()));

    assert!(!tree.verify("e", proof));
}

#[test]
fn test_tree_with_4_elements() {
    let elements = vec!["a", "b", "c", "d"];
    let mut tree = MerkleTree::new(elements).unwrap();

    assert_eq!(tree.leaves.len(), 4);
    assert_eq!(tree.root.clone().len(), 64);

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
    assert_eq!(tree.root.clone().len(), 64);

    let proof = tree.generate_proof("e").unwrap();
    assert!(tree.verify("e", proof.clone()));

    assert!(!tree.verify("f", proof));
}

#[test]
fn test_tree_with_101_elements() {
    let elements: Vec<String> = (0..101).map(|i| i.to_string()).collect();
    let element_refs: Vec<&str> = elements.iter().map(|s| s.as_str()).collect();
    let mut tree = MerkleTree::new(element_refs).unwrap();

    assert_eq!(tree.leaves.len(), 101);
    assert_eq!(tree.root.clone().len(), 64);

    let elements_to_test = vec!["0", "50", "100"];
    for &element in &elements_to_test {
        let proof = tree.generate_proof(element).unwrap();
        assert!(tree.verify(element, proof.clone()));

        let mut modified_proof = proof.clone();
        modified_proof.pop();
        assert!(!tree.verify(element, modified_proof));
    }

    assert!(tree.generate_proof("101").is_none());

    tree.add_element("new_element");
    assert_eq!(tree.leaves.len(), 102);
    assert_eq!(tree.root.clone().len(), 64);

    let proof = tree.generate_proof("new_element").unwrap();
    assert!(tree.verify("new_element", proof.clone()));

    assert!(!tree.verify("another_element", proof));
}

#[test]
fn test_tree_with_256_elements() {
    let elements: Vec<String> = (0..256).map(|i| i.to_string()).collect();
    let element_refs: Vec<&str> = elements.iter().map(|s| s.as_str()).collect();
    let mut tree = MerkleTree::new(element_refs).unwrap();

    assert_eq!(tree.leaves.len(), 256);
    assert_eq!(tree.root.clone().len(), 64);

    let elements_to_test = vec!["0", "128", "255"];
    for &element in &elements_to_test {
        let proof = tree.generate_proof(element).unwrap();
        assert!(tree.verify(element, proof.clone()));

        let mut modified_proof = proof.clone();
        modified_proof.pop();
        assert!(!tree.verify(element, modified_proof));
    }

    assert!(tree.generate_proof("256").is_none());

    tree.add_element("new_element");
    assert_eq!(tree.leaves.len(), 257);
    assert_eq!(tree.root.clone().len(), 64);

    let proof = tree.generate_proof("new_element").unwrap();
    assert!(tree.verify("new_element", proof.clone()));

    assert!(!tree.verify("another_element", proof));
}
