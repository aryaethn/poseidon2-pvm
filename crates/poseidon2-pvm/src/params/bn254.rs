//! BN254 parameter instances.
//!
//! TODO(M1): the constants below are emitted by `tools/gen-params` and checked in with a
//! provenance comment block (seed string, generation date, tool version). Until then the
//! slices are empty placeholders so the crate compiles during scaffolding.

use super::Poseidon2Params;
use crate::field::Fr;

/// Generated round constants for BN254, width 3. Replaced by `gen-params` output in M1.
static BN254_T3_RC: [Fr; 0] = [];
/// Internal-matrix diagonal for BN254, width 3. Replaced by `gen-params` output in M1.
static BN254_T3_DIAG: [Fr; 0] = [];

/// Poseidon2 over BN254 with width `t = 3` (the 2-to-1 Merkle / sponge workhorse).
///
/// `R_F = 8`, `R_P = 56` are the expected values for `t = 3`, `alpha = 5`, 128-bit
/// security; TODO(M1): confirm exactly via the parameter-generation rationale and pin.
pub static BN254_T3: Poseidon2Params = Poseidon2Params {
    width: 3,
    rounds_full: 8,
    rounds_partial: 56,
    round_constants: &BN254_T3_RC,
    mat_internal_diag: &BN254_T3_DIAG,
};
