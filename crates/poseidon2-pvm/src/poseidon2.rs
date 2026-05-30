//! The Poseidon2 permutation over BN254 `Fr`.
//!
//! Poseidon2 (Grassi, Khovratovich, Schofnegger; ePrint 2023/323) keeps Poseidon's
//! external full rounds but replaces the expensive MDS layer of the internal (partial)
//! rounds with a cheaper structured matrix, and uses an optimised external matrix. For
//! width `t = 2` / `t = 3` this is a meaningful constant-factor win over Poseidon-1, which
//! is exactly what matters when every multiply is emulated on PolkaVM.
//!
//! Round structure:
//! - **Full round:** add round constants to all elements -> S-box (`x^5`) on all elements
//!   -> multiply by the external matrix.
//! - **Partial round:** add one round constant to element 0 -> S-box on element 0 only
//!   -> multiply by the internal matrix.
//! - Schedule: `R_F/2` full rounds, then `R_P` partial rounds, then `R_F/2` full rounds.

use crate::field::Fr;
use crate::params::Poseidon2Params;

/// A configured Poseidon2 instance for a fixed width and parameter set.
pub struct Poseidon2 {
    params: Poseidon2Params,
}

impl Poseidon2 {
    /// Build an instance from a parameter set (see [`crate::params`]).
    pub fn new(params: Poseidon2Params) -> Self {
        Self { params }
    }

    /// The configured state width `t`.
    pub fn width(&self) -> usize {
        self.params.width
    }

    /// Apply the permutation in place. `state.len()` must equal `params.width`.
    pub fn permute(&self, _state: &mut [Fr]) {
        // TODO(M1):
        //   let half = self.params.rounds_full / 2;
        //   for r in 0..half                      { self.full_round(state, r); }
        //   for r in 0..self.params.rounds_partial { self.partial_round(state, r); }
        //   for r in half..self.params.rounds_full { self.full_round(state, r); }
        todo!("M1: external/internal round schedule using self.params")
    }

    /// 2-to-1 compression for binary Merkle trees: `H(left, right)`.
    pub fn hash_two(&self, _left: Fr, _right: Fr) -> Fr {
        // TODO(M1): initialise a width-`t` state with a fixed domain tag and the two
        // inputs, run `permute`, and return state[0].
        todo!("M1: fixed 2-to-1 compression")
    }
}
