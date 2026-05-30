//! Host-side algorithmic benchmarks (criterion). These measure raw wall-clock throughput of the
//! permutation and a Merkle path on the host; the funded acceptance metric is on-chain `ref_time`
//! / block-budget, measured separately in `bench-onchain/`. Both numbers are reported in M2.

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_permutation_t3(c: &mut Criterion) {
    c.bench_function("poseidon2_t3_permute", |b| {
        b.iter(|| {
            // TODO(M1/M2): construct BN254_T3 params + state, call poseidon2_pvm::poseidon2::permute.
        })
    });
}

fn bench_merkle_depth20(c: &mut Criterion) {
    c.bench_function("merkle_insert_depth20", |b| {
        b.iter(|| {
            // TODO(M2): insert into a depth-20 MerkleTree and recompute the root.
        })
    });
}

criterion_group!(benches, bench_permutation_t3, bench_merkle_depth20);
criterion_main!(benches);
