# Contributing

This project is funded under the Kusama Zero Knowledge & Advanced Cryptography Bounty and
developed in the open.

## Ground rules

- All code is MIT OR Apache-2.0.
- No unverified cryptographic constants: anything in `crates/poseidon2-pvm/src/params/`
  must be reproducible from `tools/gen-params`, with a provenance header.
- Correctness first. New hash code lands with cross-referenced test vectors (against the
  Poseidon2 reference / `ark-bn254`) before any optimisation work.
- Performance claims are reported as `ref_time` / block-budget percentage on Paseo, not as
  raw gas, because gas under-reports emulated 256-bit cost on PolkaVM.

## Before opening a PR

```bash
cargo fmt --all
cargo clippy --workspace --all-targets
cargo test --workspace
```
