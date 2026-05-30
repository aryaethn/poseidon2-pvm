//! Differential and known-answer tests.
//!
//! Field operations are checked against the `ark-bn254` reference oracle; Poseidon2 and
//! Rescue-Prime are checked against published known-answer vectors. Each test is
//! `#[ignore]`d until its `todo!()` lands, so `cargo test` stays green during scaffolding.
//! Remove the attribute as each item is implemented (M1 for field/Poseidon2, M2 for the
//! gadgets).

use poseidon2_pvm::Fr;

#[test]
#[ignore = "enable once Fr arithmetic lands (M1)"]
fn field_add_matches_arkworks() {
    // TODO(M1): for many random (a, b), assert that `Fr::add` agrees with
    // `ark_bn254::Fr` addition, compared on canonical little-endian bytes.
    let _ = Fr::ZERO;
}

#[test]
#[ignore = "enable once Fr arithmetic lands (M1)"]
fn field_mul_matches_arkworks() {
    // TODO(M1): same idea for `Fr::mul` vs the arkworks oracle.
}

#[test]
#[ignore = "enable once Fr arithmetic lands (M1)"]
fn montgomery_constants_recomputed_from_modulus() {
    // TODO(M1): recompute R, R2, INV from MODULUS and assert they match the constants
    // baked into `field.rs`, so the constants can never silently drift.
}

#[test]
#[ignore = "enable once Poseidon2 lands (M1)"]
fn poseidon2_bn254_t3_known_answer() {
    // TODO(M1): assert `hash_two` against a vector reproduced from the Poseidon2
    // reference (ePrint 2023/323) / HorizenLabs `zkhash`.
}

#[test]
#[ignore = "enable once the Merkle gadget lands (M2)"]
fn merkle_depth20_root_known_answer() {
    // TODO(M2): insert a known sequence of leaves and assert the depth-20 root.
}
