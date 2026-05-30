# Circuits (Milestone 3)

A worked **Groth16 Merkle-membership** example that uses the Poseidon2 gadget from this
repo end to end, verified on Paseo via the BN254 precompiles.

## Plan

- **Circuit:** prove knowledge of a leaf and an authentication path such that
  `MerkleRoot(leaf, path) == root`, using Poseidon2 (BN254, `t = 3`) as the 2-to-1
  compression. This is the membership core shared by airdrops, allowlists, private voting,
  and shielded-pool withdrawals.
- **Tooling:** gnark (Go) or circom + snarkjs for proving/key generation; the choice is
  recorded here once benchmarked. Witness generation reuses the Rust Merkle gadget so the
  in-circuit and on-chain hashing are guaranteed identical.
- **On-chain:** a verifier deployed to Paseo Asset Hub; verification goes through the
  BN254 `ecPairing` / `ecMul` precompiles. Deployment tx hashes are recorded for
  reproducibility.

## Deliverables

- `circuit/` — the membership circuit source.
- `keys/` — generated proving / verifying keys (or a deterministic setup script).
- `scripts/` — deterministic build + deploy + verify scripts.
- A short report: constraint count, proving time on commodity hardware, and on-chain
  verification `ref_time` / block-budget percentage.
