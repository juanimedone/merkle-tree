use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub root: Option<String>,
    pub leaves: Vec<String>,
}

impl MerkleTree {
    pub fn new(elements: Vec<&str>) -> Self {
        let leaves: Vec<String> = elements.into_iter().map(Self::hash).collect();
        let root = Self::build_tree(&leaves);
        Self { root, leaves }
    }

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

    pub fn add_element(&mut self, element: &str) {
        let new_leaf = Self::hash(element);
        self.leaves.push(new_leaf.clone());
        self.root = Self::build_tree(&self.leaves);
    }

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

    fn hash(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        hex::encode(result)
    }
}
