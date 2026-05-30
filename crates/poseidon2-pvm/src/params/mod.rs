//! Round constants and matrices for the supported hash instances.
//!
//! Every parameter set here is **generated, never hand-copied**: `tools/gen-params`
//! derives the round constants (Grain LFSR, exactly as the Poseidon/Poseidon2 spec
//! prescribes) and the external / internal matrices deterministically, then emits the
//! source files in this module together with a provenance header. Re-running the tool and
//! diffing the output is how a reviewer verifies the checked-in constants. This
//! reproducibility is the Milestone 1 deliverable.

use crate::field::Fr;

mod bn254;
pub use bn254::BN254_T3;

/// A Poseidon2 parameter set for one (field, width) instance.
///
/// Backed by `'static` slices so an instance is a cheap `Copy` value that can live in a
/// `static`/`const` and be embedded directly in the PolkaVM artifact.
#[derive(Clone, Copy)]
pub struct Poseidon2Params {
    /// State width `t` (e.g. 3 for the 2-to-1 Merkle compression).
    pub width: usize,
    /// Number of full rounds `R_F` (split half before and half after the partial rounds).
    pub rounds_full: usize,
    /// Number of partial rounds `R_P`.
    pub rounds_partial: usize,
    /// Round constants, laid out per the round schedule (generated).
    pub round_constants: &'static [Fr],
    /// Diagonal of the internal (partial-round) matrix, per Poseidon2 (generated).
    pub mat_internal_diag: &'static [Fr],
}
