# On-chain `ref_time` benchmark harness (Milestone 2)

The funded acceptance metric is **`ref_time` / block-budget percentage**, not gas, because gas on
`pallet-revive` understates emulated 256-bit arithmetic. This harness deploys the contract to a
local node (`ink-node`, which ships `pallet-revive`) and/or Paseo Asset Hub, exercises each
operation, and records the metered `ref_time` and `proof_size`.

## What it measures

- Single Poseidon2 hash (t=2 compression, t=3 sponge).
- A depth-20 and depth-32 Merkle `insertLeaf`.
- For comparison: the existing Poseidon-1 PVM implementation and (where it fits) in-Solidity
  Poseidon, reproducing the study's baseline so the improvement is apples-to-apples.

## Method

1. Build + link the `merkle-contract` (PolkaVM toolchain).
2. Deploy to the target node.
3. Call each entry point; read the dispatched weight (`ref_time`, `proof_size`) from the
   transaction result, and convert `ref_time` to a block-budget percentage
   (block budget ~= 2.06e12 ps; one in-Solidity Poseidon ~= 70%).
4. Emit a Markdown table (gas, `ref_time`, block %) committed alongside the report.

See `measure_reftime.sh` for the scaffold.
