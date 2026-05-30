//! A sponge over a fixed-width permutation, with explicit domain separation (Milestone 2).

use crate::field::Fr;
use alloc::vec::Vec;

/// Sponge state parameterised by rate and capacity. A domain separator is mixed into the
/// capacity at initialisation so that hashes of different shapes (e.g. a 2-input Merkle
/// node vs. an n-input commitment) can never collide.
pub struct Sponge {
    // TODO(M2): rate, capacity, the running state, and an absorb position.
    _private: (),
}

impl Sponge {
    /// Create a sponge with the given rate, capacity, and domain separator.
    pub fn new(_rate: usize, _capacity: usize, _domain: u64) -> Self {
        todo!("M2: initialise capacity lane(s) with the domain separator")
    }

    /// Absorb input, permuting at each rate boundary.
    pub fn absorb(&mut self, _input: &[Fr]) {
        todo!("M2: absorb")
    }

    /// Squeeze `n` field elements out of the sponge.
    pub fn squeeze(&mut self, _n: usize) -> Vec<Fr> {
        todo!("M2: squeeze")
    }
}
