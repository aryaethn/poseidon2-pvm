//! Native (host) micro-benchmarks via Criterion.
//!
//! These measure *algorithmic* cost on the host and are handy for catching regressions
//! while implementing M1/M2. They are NOT the funded performance metric: the headline
//! numbers are `ref_time` / block-budget measurements taken on Paseo via the contract
//! crate and the M2 benchmark harness. (Benches will panic until the `todo!()`s land.)

use criterion::{criterion_group, criterion_main, Criterion};
use poseidon2_pvm::{params::BN254_T3, Fr, Poseidon2};

fn bench_hash_two(c: &mut Criterion) {
    let hasher = Poseidon2::new(BN254_T3);
    let a = Fr::ZERO;
    let b = Fr::ONE;
    c.bench_function("poseidon2_hash_two", |bencher| {
        bencher.iter(|| hasher.hash_two(a, b))
    });
}

criterion_group!(benches, bench_hash_two);
criterion_main!(benches);
