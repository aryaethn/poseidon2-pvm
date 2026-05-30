//! pallet-revive PVM contract: an on-chain incremental Poseidon2 Merkle tree.
//!
//! Exposes (Ethereum-style 4-byte selectors; see `abi/MerkleTree.sol`):
//!   * `insertLeaf(bytes32 leaf) -> uint64 index`  — append a leaf, update the root.
//!   * `getRoot() -> bytes32`                       — current Merkle root.
//!
//! Notes:
//! * `pallet-revive` has no heap allocator by default; this contract uses only fixed-size
//!   buffers. Storage is a raw 32-byte-key / bytes-value KV store via `pallet-revive-uapi`.
//! * The deposit path is exactly where the feasibility study showed in-Solidity Poseidon is
//!   impossible and Rust-on-PVM is required.
//! * Confirm the crate-level attributes, entry-point export macro, and host-fn names against
//!   `paritytech/rust-contract-template` for your polkadot-sdk revision — the API below is a
//!   scaffold, deliberately not pinned to a possibly-stale signature.

#![no_std]
#![no_main]

// use pallet_revive_uapi::{HostFn, HostFnImpl as api, StorageFlags};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // pallet-revive aborts on panic; keep it minimal and allocation-free.
    loop {}
}

/// Contract constructor: initialize an empty depth-20 tree root in storage.
#[no_mangle]
// #[polkavm_derive::polkavm_export]   // confirm exact export macro against the template
extern "C" fn deploy() {
    // TODO(M3): build MerkleTree::<20>::new(), store its root and the zeros ladder.
}

/// Contract entry point: read the 4-byte selector and dispatch.
#[no_mangle]
// #[polkavm_derive::polkavm_export]
extern "C" fn call() {
    // TODO(M3):
    //   1. read call input via the uapi host fn; first 4 bytes = selector.
    //   2. match selector:
    //        insertLeaf -> decode bytes32 leaf -> Fr -> MerkleTree::insert -> persist -> return index
    //        getRoot    -> load root -> return bytes32
    //   3. on unknown selector, revert.
    let _ = poseidon2_pvm::Fr::ZERO; // links the core crate; replace with real logic.
}
