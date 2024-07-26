use sha2::{Digest, Sha256};

/// Enum representing the sibling position in the Merkle Tree.
#[derive(Debug, Clone)]
pub enum Sibling {
    Left(String),
    Right(String),
}

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
    /// An `Option<Vec<Sibling>>` representing the proof, where each `Sibling` indicates whether the hash is from the left or right sibling.
    pub fn generate_proof(&self, element: &str) -> Option<Vec<Sibling>> {
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
                        Sibling::Right(right.clone())
                    } else {
                        Sibling::Left(left.clone())
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
    /// * `proof` - A vector of `Sibling` representing the proof.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the element is part of the tree.
    pub fn verify(&self, element: &str, proof: Vec<Sibling>) -> bool {
        if let Some(root) = &self.root {
            let mut hash = Self::hash(element);
            for sibling in proof {
                hash = match sibling {
                    Sibling::Left(s) => Self::hash(&(s + &hash)),
                    Sibling::Right(s) => Self::hash(&(hash + &s)),
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
        let result = Sha256::digest(data);
        hex::encode(result)
    }
}
