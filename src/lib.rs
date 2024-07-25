use sha2::{Digest, Sha256};

/// A struct representing a Merkle Tree.
#[derive(Debug, Clone)]
pub struct MerkleTree {
    /// The root hash of the Merkle Tree. It is `None` if tree is empty.
    pub root: Option<String>,
    /// The leaf nodes of the Merkle Tree.
    pub leaves: Vec<String>,
}

impl MerkleTree {
    /// Creates a new Merkle Tree from a vector of string elements.
    ///
    /// # Arguments
    ///
    /// * `elements` - A vector of string slices representing the elements to be included in the tree.
    ///
    /// # Returns
    ///
    /// A new `MerkleTree` instance.
    pub fn new(elements: Vec<&str>) -> Self {
        let leaves: Vec<String> = elements.into_iter().map(Self::hash).collect();
        let root = Self::build_tree(&leaves);
        Self { root, leaves }
    }

    /// Builds the Merkle Tree from the given leaves.
    ///
    /// # Arguments
    ///
    /// * `leaves` - A slice of strings representing the leaf nodes.
    ///
    /// # Returns
    ///
    /// An `Option<String>` representing the root hash of the tree. It is `None` if tree is empty.
    fn build_tree(leaves: &[String]) -> Option<String> {
        if leaves.is_empty() {
            return None;
        }
        let mut current_level = leaves.to_vec();
        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            for i in (0..current_level.len()).step_by(2) {
                let left = &current_level[i];
                let right = if i + 1 < current_level.len() {
                    &current_level[i + 1]
                } else {
                    left
                };
                let parent_hash = Self::hash(&(left.clone() + right));
                next_level.push(parent_hash);
            }
            current_level = next_level;
        }
        Some(current_level[0].clone())
    }

    /// Adds a new leaf element and builds the Merkle Tree again.
    ///
    /// # Arguments
    ///
    /// * `element` - A string slice representing the new element to be added.
    pub fn add_element(&mut self, element: &str) {
        let new_leaf = Self::hash(element);
        self.leaves.push(new_leaf.clone());
        self.root = Self::build_tree(&self.leaves);
    }

    /// Generates a proof that an element is contained in the Merkle Tree.
    ///
    /// # Arguments
    ///
    /// * `element` - A string slice representing the element for which the proof is to be generated.
    ///
    /// # Returns
    ///
    /// An `Option<Vec<(String, bool)>>` representing the proof, where each tuple contains a hash
    /// and a boolean indicating whether the hash is from the left (true) or right (false) sibling.
    pub fn generate_proof(&self, element: &str) -> Option<Vec<(String, bool)>> {
        if self.leaves.is_empty() {
            return None;
        }

        let mut hash = Self::hash(element);
        let mut proof = Vec::new();
        let mut current_level = self.leaves.clone();

        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            let mut found = false;

            for i in (0..current_level.len()).step_by(2) {
                let left = &current_level[i];
                let right = if i + 1 < current_level.len() {
                    &current_level[i + 1]
                } else {
                    left
                };
                let parent_hash = Self::hash(&(left.clone() + right));
                if left == &hash || right == &hash {
                    proof.push(if left == &hash {
                        (right.clone(), false)
                    } else {
                        (left.clone(), true)
                    });
                    hash.clone_from(&parent_hash);
                    found = true;
                }
                next_level.push(parent_hash);
            }

            if !found {
                return None;
            }

            current_level = next_level;
        }

        Some(proof)
    }

    /// Verifies that an element is part of the Merkle Tree using the given proof.
    ///
    /// # Arguments
    ///
    /// * `element` - A string slice representing the element to be verified.
    /// * `proof` - A vector of tuples representing the proof, where each tuple contains a hash
    ///   and a boolean indicating whether the hash is from the left (true) or right (false) sibling.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the element is part of the tree.
    pub fn verify(&self, element: &str, proof: Vec<(String, bool)>) -> bool {
        if let Some(root) = &self.root {
            let mut hash = Self::hash(element);
            for (p, is_left) in proof {
                hash = if is_left {
                    Self::hash(&(p + &hash))
                } else {
                    Self::hash(&(hash + &p))
                };
            }
            hash == *root
        } else {
            false
        }
    }

    /// Computes the SHA256 hash of the given data.
    ///
    /// # Arguments
    ///
    /// * `data` - A string slice representing the data to be hashed.
    ///
    /// # Returns
    ///
    /// A string representing the hexadecimal encoding of the SHA256 hash of the data.
    fn hash(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        hex::encode(result)
    }
}
