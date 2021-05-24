// rug is the Rust equivalent of GMP (GNU Multi Precision Arithmetic Library)
// so this is the equiv of "#include <gmp.h>" in the pysloth implementation
// we use this since Rust does not support 256bit integers
use rand::Rng;
use rug::ops::NegAssign;
use rug::{integer::IsPrime, integer::Order, Assign, Integer};
use std::ops::AddAssign;

// for a prime size of 256 bits, derive the LARGEST prime and resulting exponent
// for a given input block of 256 bits (akin the the plaintext), compute the modular square root (akin to the ciphertext)
// for a given output block of 256 bits (akin the to ciphertext) compute the inverse square root (which should result in the plain text)
// so PT(256 bits) -> CT -> PT(256 bits)

// block size of 256 bits
const BLOCK_BYTE_SIZE: usize = 32;
// prime size of 256 bits
const PRIME_BYTE_SIZE: usize = 32;
// num iterations used in pysloth implementation (https://github.com/randomchain/pysloth/blob/master/sloth.c)
const PRIME_CHECK_ITERS: u32 = 25;
const ORDER: Order = Order::Lsf;
type Block = [u8; BLOCK_BYTE_SIZE];

fn from_block(block: Block) -> Integer {
    let from = Integer::from_digits(&block, ORDER);
    from
}

fn to_block(int: Integer) -> Block {
    let mut block: Block = [0; BLOCK_BYTE_SIZE];
    int.write_digits(&mut block, ORDER);
    block
}

// find next lowset prime above a given value
fn next_prime(p: &mut Integer) {
    if p.is_even() {
        *p += Integer::from(1);
    } else {
        *p += Integer::from(2);
    }

    while p.is_probably_prime(PRIME_CHECK_ITERS) == IsPrime::No {
        *p += Integer::from(2);
    }
}

// find next highest prime below a given value
fn prev_prime(p: &mut Integer) {
    if p.is_even() {
        *p -= Integer::from(1);
    } else {
        *p -= Integer::from(2);
    }

    while p.is_probably_prime(PRIME_CHECK_ITERS) == IsPrime::No {
        *p -= Integer::from(2);
    }
}

fn gen_largest_prime(max_size_bytes: usize) -> Integer {
    let mut prime = Integer::from(Integer::u_pow_u(2, (max_size_bytes * 8) as u32)) - 1;
    prev_prime(&mut prime);
    // ensure prime is congruent to 3 mod 4
    // as specified in paper
    while prime.mod_u(4) != 3 {
        prev_prime(&mut prime);
    }
    prime
}

fn run() {
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
    println!("\n");
    println!("block: {:?}", block_in);
    println!("input: {:?}", int);
    println!("exponent: {:?}", exponent);
    println!("prime: {:?}", prime);
    println!("perm: {:?}", perm);
    println!("inv: {:?}", inv);
    let block_out = to_block(inv);
    println!("block: {:?}", block_out);
}

fn sqrt_permutation(input: &Integer, exp: &Integer, prime: &Integer) -> Integer {
    let mut result = Integer::from(0);

    if input.legendre(prime) == 1 {
        let mut tmp = input.clone();
        tmp = match tmp.pow_mod(exp, prime) {
            Ok(tmp) => tmp,
            Err(_) => unreachable!(),
        };
        if tmp.is_even() {
            result.assign(tmp);
        } else {
            tmp.neg_assign();
            tmp.add_assign(prime);
            result.assign(tmp);
        }
    } else {
        let mut tmp = Integer::from(prime - input);
        tmp = match tmp.pow_mod(exp, prime) {
            Ok(tmp) => tmp,
            Err(_) => unreachable!(),
        };
        if tmp.is_odd() {
            result.assign(tmp);
        } else {
            tmp.neg_assign();
            tmp.add_assign(prime);
            result.assign(tmp);
        }
    }

    result
}

fn inverse_sqrt(input: &Integer, prime: &Integer) -> Integer {
    let mut result = Integer::from(0);
    let tmp = input.clone();
    let square = tmp.square();

    if input.is_even() {
        let tmp = match square.pow_mod(&Integer::from(1), prime) {
            Ok(tmp) => tmp,
            Err(_) => unreachable!(),
        };
        result.assign(tmp);
    } else {
        let mut tmp = match square.pow_mod(&Integer::from(1), prime) {
            Ok(tmp) => tmp,
            Err(_) => unreachable!(),
        };
        tmp.neg_assign();
        tmp.add_assign(prime);
        result.assign(tmp);
    }

    result
}

////////////////////////////////////////////////////////////////////////////////////

fn main() {
    let block_a: Block = rand::random();
    println!("{:?}", block_a);
    let int = from_block(block_a);
    println!("{:?}", int);
    let block_b = to_block(int);
    println!("{:?}", block_b);
    assert_eq!(block_a, block_b);

    run();
}

// creating this submodule means we won't compile testing code
// when we compile "production-ready" binaries
#[cfg(test)]
mod test {
    // import all code outside of this submodule
    use super::*;
    use rand;
    use rug::{integer::IsPrime, Integer};

    #[test]
    fn test_conversion() {
        // generate random block
        for _n in 0..100 {
            let block_a: Block = rand::random();
            let int = from_block(block_a);
            let block_b = to_block(int);
            assert_eq!(block_a, block_b);
        }
    }

    #[test]
    fn test_prime_generation() {
        // test for many byte size configs
        for size in 1..64 {
            let mut prime = gen_largest_prime(size);
            assert_ne!(prime.is_probably_prime(PRIME_CHECK_ITERS), IsPrime::No);

            // ensure the next prime is larger than 2^256 - 1 OR is not congruent to 4 mod 3
            next_prime(&mut prime);
            let largest_value = Integer::from(Integer::u_pow_u(2, (size * 8) as u32)) - 1;
            assert!(prime > largest_value || prime.mod_u(4) != 3);
        }
    }

    #[test]
    fn test_largest_prime_256_bits() {
        // test for many byte size configs
        let prime = gen_largest_prime(PRIME_BYTE_SIZE);
        assert_ne!(prime.is_probably_prime(PRIME_CHECK_ITERS), IsPrime::No);

        // 2^256 - 189 | largest prime that fits into 256 bits
        let largest_prime_str =
            "115792089237316195423570985008687907853269984665640564039457584007913129639747";
        assert_eq!(prime, largest_prime_str.parse::<Integer>().unwrap());
    }

    #[test]
    fn test_end_to_end() {
        for _n in 0..100 {
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
            let block_out = to_block(inv);

            assert_eq!(block_in, block_out);
        }
    }
}
