//! Incremental binary Merkle tree over the 2-to-1 Poseidon2 compression (Milestone 2).
//!
//! Supports depth-20 (1M leaves) and depth-32 trees with cached filled subtrees and
//! incremental insertion, matching the shape every shielded-pool / membership circuit
//! needs. The motivation is the feasibility-study result that a depth-20 Poseidon path is
//! impossible in Solidity but ~37.9k gas in hand-written Rust on PolkaVM.

use crate::field::Fr;
use crate::poseidon2::Poseidon2;
use alloc::vec::Vec;

/// An append-only Merkle tree bound to a Poseidon2 hasher.
pub struct MerkleTree<'a> {
    hasher: &'a Poseidon2,
    depth: usize,
    // TODO(M2): filled-subtree cache, per-level zero hashes, next index, current root.
}

impl<'a> MerkleTree<'a> {
    /// Create an empty tree of the given depth. Precomputes the per-level zero hashes.
    pub fn new(hasher: &'a Poseidon2, depth: usize) -> Self {
        // TODO(M2): precompute zero hashes z[0] = 0, z[i] = H(z[i-1], z[i-1]).
        Self { hasher, depth }
    }

    /// The configured depth.
    pub fn depth(&self) -> usize {
        self.depth
    }

    /// Insert a leaf, returning its index. `O(depth)` hashes via cached subtrees.
    pub fn insert(&mut self, _leaf: Fr) -> u64 {
        let _ = self.hasher; // bound for M2 implementation
        todo!("M2: incremental insertion with cached subtrees")
    }

    /// The current Merkle root.
    pub fn root(&self) -> Fr {
        todo!("M2: current root")
    }

    /// The authentication path (sibling hashes) for a previously inserted leaf index.
    pub fn proof(&self, _index: u64) -> Vec<Fr> {
        todo!("M2: sibling path")
    }
}
