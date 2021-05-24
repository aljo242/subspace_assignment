use rug::{Integer};
use subspace_assignment::{Block, gen_largest_prime, from_block, to_block, sqrt_permutation, inverse_sqrt, PRIME_BYTE_SIZE};



/// perform a basic run through of the block encoding pipeline
/// 1. create the largest prime that is congruent to 4 mod 3
/// 2. derive exponent from prime using e = (prime + 1) / 4
/// 3. create a random block of data to be "encoded"
/// 4. find the square root permutation ("encode" to cipher text)
/// 5. find the inverse sqrt ("decode" back to plain text)
/// 6. verify that the encoding preserved the data
fn main() {
    println!("Performing basic run-through...");
    // generate largest prime
    // create largest number possible for PRIME_BYTE_SIZE and then reduce
    // until it is largest viable PRIME
    let prime = gen_largest_prime(PRIME_BYTE_SIZE);

    // e = (p + 1) / 4
    let mut exponent = prime.clone() + Integer::from(1);
    exponent.div_exact_u_mut(4);

    // generate random number as input
    let block_in: Block = rand::random();
    let int = from_block(block_in);
    let perm = sqrt_permutation(&int, &exponent, &prime);
    let inv = inverse_sqrt(&perm, &prime);
    println!("input block: {:?}", block_in);
    println!("sqrt perm: {:?}", perm);
    println!("inverse sqrt: {:?}", inv);
    let block_out = to_block(inv);
    println!("output block: {:?}", block_out);
    assert_eq!(block_in, block_out);
}