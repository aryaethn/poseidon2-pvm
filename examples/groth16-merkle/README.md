# Groth16 Merkle-membership integration (Milestone 3)

End-to-end demo proving that the suite plugs into the BN254 / Groth16 path that Asset Hub
verifies cheaply.

## Flow

1. **On-chain tree.** The `merkle-contract` maintains a Poseidon2 incremental Merkle tree;
   `insertLeaf` appends commitments, `getRoot` returns the current root.
2. **Off-chain proof.** A circuit proves *knowledge of a leaf and a Merkle path to a public
   root*, using a **Poseidon2 gadget that matches the on-chain parameters exactly**. Two routes:
   - **circom + snarkjs** (Groth16, BN254) with a Poseidon2 template, or
   - **gnark** (Go, Groth16, BN254) with a Poseidon2 hash gadget.
3. **On-chain verification.** The Groth16 proof is verified via the BN254 precompiles
   (`ecPairing` / `ecMul`) — ~$0.017 per the feasibility study — with the public root checked
   against `getRoot`.

## Critical correctness invariant

The **in-circuit** Poseidon2 and the **on-chain** Poseidon2 (`poseidon2-pvm`) must use
byte-for-byte identical parameters, width, and domain separation. If they differ, the computed
roots will not match and proofs will fail to verify. The generated parameters from `params-gen`
are the single source of truth for both sides; document the exact set used here.

## Deliverables (M3)

- `circuit/`  — the membership circuit (circom or gnark) + proving/verifying keys.
- `verifier/` — verifier wired through the BN254 precompiles, deployed to Paseo.
- `scripts/`  — deterministic deploy + an end-to-end "deposit -> prove -> verify" test on Paseo,
  with recorded transaction hashes.
