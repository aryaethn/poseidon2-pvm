//! Rescue-Prime over BN254 `Fr` (Milestone 2).
//!
//! Offered as an alternative to Poseidon2 for designs that prefer Rescue's security
//! margins. Each round alternates the S-box `x^5` and its inverse `x^{1/5}`, which is more
//! expensive to evaluate but arguably simpler to analyse; the trade-off is quantified in
//! the M2 `ref_time` benchmark report.

use crate::field::Fr;

/// A configured Rescue-Prime instance.
pub struct RescuePrime {
    // TODO(M2): width, rounds, MDS matrix, round constants, alpha and alpha_inv.
    _private: (),
}

impl RescuePrime {
    /// Apply the permutation in place (Milestone 2).
    pub fn permute(&self, _state: &mut [Fr]) {
        todo!("M2: Rescue-Prime round function (forward and inverse S-box layers)")
    }
}
