# poseidon2-pvm

**A hand-written, PolkaVM-tuned ZK-friendly hash suite (Poseidon2 + Rescue-Prime) over the
BN254 scalar field, with sponge and incremental Merkle gadgets.**

> Status: **work in progress** (scaffolding). APIs are stubbed and filled in across three
> milestones. **Not audited; do not use in production yet.**
>
> Funded / Secured / Supported by **Kusama Network** (Zero Knowledge & Advanced
> Cryptography Bounty).

---

## Why this exists

Kusama Asset Hub runs `pallet-revive`, a RISC-V / PolkaVM target. On it, generic 256-bit
field arithmetic is *emulated* and dominates `ref_time` (the real block-budget metric, of
which gas is a misleading proxy). The first public benchmark of ZK operations on the
platform found that:

- BN254 precompiles are cheap, so **Groth16 verification (~$0.017) is the native fit**.
- A single **Poseidon hash in Solidity eats ~70% of a block's `ref_time`** budget, and a
  **depth-20 Merkle path is impossible**.
- A hand-written **Rust Poseidon compiled to PolkaVM** makes that same depth-20 path
  feasible (~37.9k gas, ~28x `ref_time` improvement over the Solidity path).

That breakthrough exists today only as a single **Poseidon-1** point implementation. This
crate generalises it to the faster **Poseidon2** permutation (the modern default), adds a
**Rescue-Prime** alternative, ships the **sponge and incremental Merkle** gadgets every
shielded-pool / membership / credential circuit needs, and makes the parameters **fully
reproducible** so nothing is an unaudited magic constant.

## What it provides

- `Fr` — BN254 scalar field, four `u64` limbs in Montgomery form, tuned for RISC-V 64-bit.
- `Poseidon2` — the permutation and a 2-to-1 compression for Merkle trees (`t = 2`, `t = 3`).
- `RescuePrime` — an alternative permutation (M2).
- `Sponge` — domain-separated sponge over a permutation (M2).
- `MerkleTree` — incremental binary Merkle tree, depth-20 / depth-32, cached subtrees (M2).
- `tools/gen-params` — deterministic, reproducible parameter generation (M1).
- `crates/poseidon2-pvm-contract` — the on-chain wrapper for pallet-revive (M3).
- `circuits/` — a worked Groth16 Merkle-membership example verified on Paseo (M3).

## Layout

```text
.
├── crates/
│   ├── poseidon2-pvm/              # core no_std library (the deliverable)
│   │   ├── src/
│   │   │   ├── field.rs            # BN254 Fr, Montgomery u64x4
│   │   │   ├── poseidon2.rs        # permutation + 2-to-1 compression
│   │   │   ├── rescue.rs           # Rescue-Prime (M2)
│   │   │   ├── sponge.rs           # sponge (M2)
│   │   │   ├── merkle.rs           # incremental Merkle (M2)
│   │   │   └── params/             # generated round constants + matrices
│   │   ├── tests/vectors.rs        # differential + known-answer tests
│   │   └── benches/hashing.rs      # native Criterion micro-benchmarks
│   └── poseidon2-pvm-contract/     # PolkaVM / pallet-revive wrapper (M3)
├── tools/gen-params/               # deterministic parameter generator (M1)
├── circuits/                       # Groth16 integration example (M3)
└── .github/workflows/ci.yml        # fmt + clippy + build + test
```

## Build & test

```bash
# format once before your first commit so the CI fmt check passes
cargo fmt --all

cargo build --workspace
cargo test  --workspace          # scaffolding tests are #[ignore]d until implemented
cargo bench -p poseidon2-pvm     # native micro-benchmarks (after M1/M2)
```

The PolkaVM contract build (Milestone 3) layers the pallet-revive / PolkaVM target on top
of the pinned stable toolchain; exact steps are documented in
`crates/poseidon2-pvm-contract` as that milestone lands.

## Roadmap (mapped to the funded milestones)

| Milestone | Scope |
|-----------|-------|
| **M1 — Poseidon2 core** | `Fr` arithmetic (CIOS Montgomery), Poseidon2 (`t = 2/3`), reproducible parameter generation, cross-checked test vectors, no_std / PVM build. |
| **M2 — Gadgets + Rescue + benchmarks** | Sponge, incremental Merkle (depth-20/32), Rescue-Prime, and a `ref_time` / block-budget benchmark harness vs the existing Poseidon-1 PVM implementation and vs Solidity. |
| **M3 — Groth16 integration + release** | Worked Merkle-membership circuit verified on Paseo via the BN254 precompiles, full docs, crate release, demo video. |

## Security

Pre-audit and incomplete. The S-box composition is correct by construction once the field
ops land, but parameters, constant-time properties (not required here: all exponents are
public), and serialisation edge cases are validated as part of M1/M2. Do not deploy with
real value until a review has been completed.

## License

Licensed under either of [MIT](./LICENSE-MIT).
