use criterion::{criterion_group, criterion_main, Criterion, black_box};
use subspace_assignment::{inverse_sqrt, sqrt_permutation, run, from_block, gen_largest_prime, PRIME_BYTE_SIZE, Block};
use rug::{Integer};

fn encode_decode(c: &mut Criterion) {
    let prime = gen_largest_prime(PRIME_BYTE_SIZE);

    // e = (p + 1) / 4
    let mut exponent = prime.clone() + Integer::from(1);
    exponent.div_exact_u_mut(4);

    let block_in: Block = rand::random();
    let int = from_block(block_in);

    c.bench_function("encode", |b| b.iter(|| 
        black_box(sqrt_permutation(&int, &exponent, &prime))
    ));

    let perm = sqrt_permutation(&int, &exponent, &prime); 

    c.bench_function("decode", |b| b.iter(|| 
        black_box(inverse_sqrt(&perm, &prime))
    ));

}

fn end_to_end(c: &mut Criterion) {
    c.bench_function("end_to_end", |b| b.iter(|| 
        black_box(run())
    ));
}
  

criterion_group!(benches, end_to_end, encode_decode);
criterion_main!(benches);

// paper: C = 77.922
// mine: C = 22.963 us / 319.4 ns = 71.7

