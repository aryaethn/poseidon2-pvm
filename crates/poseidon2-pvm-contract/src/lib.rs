//! On-chain wrapper (Milestone 3).
//!
//! This crate is what gets compiled to PolkaVM and deployed to Kusama Asset Hub
//! (`pallet-revive`). It exposes the `poseidon2-pvm` primitives behind a minimal ABI so a
//! Solidity logic contract can call hand-written Rust hashing as a precompile-style
//! helper, following the hybrid architecture from the feasibility study (Solidity logic
//! + Rust Poseidon + BN254 Groth16 precompiles).
//!
//! The exact entry-point macro and ABI come from the Polkadot SDK's `pallet-revive`
//! tooling and are wired up in Milestone 3. See
//! <https://docs.polkadot.com/smart-contracts/precompiles/>.

#![cfg_attr(target_arch = "riscv64", no_std)]

use poseidon2_pvm::{params::BN254_T3, Fr, Poseidon2};

/// ABI-facing 2-to-1 hash: takes two canonical little-endian field encodings and returns
/// `Poseidon2(left, right)` as canonical little-endian bytes.
///
/// On the host this is a plain function (unit-testable); in M3 it is also surfaced through
/// the `pallet-revive` contract ABI.
pub fn hash_two_le(left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
    // TODO(M3): finalise error handling for non-canonical inputs at the ABI boundary.
    let l = Fr::from_bytes_le(&left).expect("left input must be a canonical field element");
    let r = Fr::from_bytes_le(&right).expect("right input must be a canonical field element");
    let hasher = Poseidon2::new(BN254_T3);
    hasher.hash_two(l, r).to_bytes_le()
}
