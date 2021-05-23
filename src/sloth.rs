// rug is the Rust equivalent of GMP (GNU Multi Precision Arithmetic Library)
// so this is the equiv of "#include <gmp.h>" in the pysloth implementation
// we use this since Rust does not support 256bit integers
use rug::{Assign, Integer};

// for a prime size of 256 bits, derive the LARGEST prime and resulting exponent
// for a given input block of 256 bits (akin the the plaintext), compute the modular square root (akin to the ciphertext)
// for a given output block of 256 bits (akin the to ciphertext) compute the inverse square root (which should result in the plain text)
// so PT(256 bits) -> CT -> PT(256 bits)


// find the next prime
pub fn next_prime(p: &mut Integer, n: Integer) {

}

pub fn dummy(p: &mut Integer, n: Integer) {
    p.assign(10);
    let decimal = "98_765_432_109_876_543_210";
    p.assign(Integer::parse(decimal).unwrap());
    p.assign(n);    
}