# Parameter provenance

Reproducibility is a grant success metric: every constant in `poseidon2-pvm` is regenerable from
a documented, seeded procedure via `params-gen`, not hand-committed magic.

## Instance descriptor

| Field            | Value                                  |
|------------------|----------------------------------------|
| Field            | BN254 scalar field `Fr`                |
| S-box degree `d` | 5 (since gcd(5, r-1) = 1)              |
| Widths `t`       | 2 (2-to-1 compression), 3 (sponge)     |
| `R_F` (full)     | 8                                      |
| `R_P` (partial)  | 56 (t=2), 56 (t=3) — verify vs reference |

## Generation method (Milestone 1)

- **Round constants:** Grain LFSR seeded with the standard Poseidon parameter descriptor
  (prime, `d`, `t`, `R_F`, `R_P`), matching the reference so output agrees with independent
  implementations (Poseidon2 reference / HorizenLabs `zkhash`).
- **External matrix `M_E`:** the Poseidon2 small-width MDS for `t in {2,3}`; verified MDS.
- **Internal diagonal `mu`:** per the Poseidon2 construction; checked for invertibility and the
  required round-structure security properties.

## Verification

`params-gen` prints the seed and a digest of its output. Re-running it must reproduce the
committed `params_generated.rs` byte-for-byte. The same parameters MUST be used by any in-circuit
Poseidon2 gadget (see `examples/groth16-merkle`) or Merkle roots will not match.

## References

- Poseidon2: ePrint 2023/323.
- Kusama Asset Hub ZK feasibility study: `codeberg.org/hantoniu/zk-benchmarks`.
