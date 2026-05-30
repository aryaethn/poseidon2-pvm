#![no_std]
#![forbid(unsafe_code)]
//! # poseidon2-pvm
//!
//! A hand-written, PolkaVM-tuned suite of ZK-friendly hash functions over the BN254
//! scalar field: **Poseidon2** (the Merkle/sponge workhorse) and **Rescue-Prime** (an
//! alternative with different security margins), plus sponge and incremental Merkle-tree
//! gadgets.
//!
//! ## Why this crate exists
//!
//! On Kusama Asset Hub (`pallet-revive`, a RISC-V / PolkaVM target), generic 256-bit
//! field arithmetic is *emulated* and dominates `ref_time`. A public benchmark found that
//! a single Poseidon hash written in Solidity consumes ~70% of a block's `ref_time`
//! budget, and a depth-20 Merkle path is outright impossible; a hand-written Rust
//! implementation compiled to PolkaVM makes the same path feasible (~37.9k gas). This
//! crate generalises that result to the faster Poseidon2 permutation and ships the
//! surrounding gadgets so downstream privacy applications (shielded pools, membership
//! proofs, anonymous credentials) can build on one audited, reproducible primitive.
//!
//! ## Status
//!
//! Work in progress. APIs are stubbed with `todo!()` and filled in across three
//! milestones (M1: Poseidon2 core; M2: gadgets + Rescue-Prime + benchmarks; M3: Groth16
//! integration). NOT audited; do not use in production yet.
//!
//! ---
//! Funded / Secured / Supported by **Kusama Network**.

extern crate alloc;

pub mod field;
pub mod merkle;
pub mod params;
pub mod poseidon2;
pub mod rescue;
pub mod sponge;

pub use field::Fr;
pub use params::Poseidon2Params;
pub use poseidon2::Poseidon2;
