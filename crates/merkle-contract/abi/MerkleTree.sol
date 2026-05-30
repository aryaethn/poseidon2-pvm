// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.0;

/// @notice ABI stub for callers of the Rust PVM Merkle contract.
/// Lets Solidity logic contracts (or viem/ethers clients) encode calls to the Rust-built
/// contract that maintains the Poseidon2 Merkle tree on Asset Hub.
interface IMerkleTree {
    /// @notice Append a leaf and return its index.
    function insertLeaf(bytes32 leaf) external returns (uint64 index);

    /// @notice Current Merkle root.
    function getRoot() external view returns (bytes32 root);
}
