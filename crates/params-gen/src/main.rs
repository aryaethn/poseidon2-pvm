//! Deterministic parameter generator (host, std).
//!
//! Emits round constants, the external MDS matrix `M_E`, and the internal diagonal `mu` for a
//! given width, as a committed `params_generated.rs`. Every value is derived from a documented,
//! seeded procedure so a reviewer can regenerate and diff (a grant success metric).
//!
//! Method (Milestone 1):
//!   * Round constants: Grain LFSR seeded with the standard Poseidon parameter descriptor
//!     (field prime, S-box degree d=5, width t, R_F, R_P), exactly as in the reference, so the
//!     output matches independent implementations.
//!   * `M_E`: an MDS matrix for width t per the Poseidon2 construction (e.g. the small fixed MDS
//!     for t in {2,3}); verified MDS.
//!   * `mu`: internal-matrix diagonal per the Poseidon2 construction; checked for invertibility
//!     and the required security properties.
//!
//! Usage: `cargo run -p params-gen -- --width 3 --out ../poseidon2-pvm/src/poseidon2/params_generated.rs`

fn main() {
    // TODO(M1): parse --width / --out, run Grain LFSR + matrix construction, write the file,
    // and print the seed + a digest of the output so provenance is logged.
    eprintln!("params-gen: not yet implemented (Milestone 1). See docs/parameters.md.");
}
