use criterion::{criterion_group, criterion_main, Criterion};
use subspace_assignment::run;


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("256 bit", |b| b.iter(|| run()));
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
