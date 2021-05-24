# Alex Johnson Subspace Coding Assignment

## Installation

Clone the repository and build (debug).

```bash
git clone https://github.com/aljo242/subspace_assignment.git
cd subspace_assignment
cargo build
```
## Release Build

```bash
cargo build --release
```

## Testing

```bash
cargo test
```

## Benchmarking

```bash
cargo bench
```

## Description

In Subspace, we adapt the underlying modular square root permutation to create a
block cipher. This ensures farmers expend some minimum wall-clock time T (and
proportional computation) while encoding each piece, while verifiers only have to
expend time T/C to decode each piece, where C is a constant speed-up proportional
to the size of the Prime used.

## Assignment:

Port the underlying square root permutation to Rust using the C implementation as
a guide. Please note that you are not being asked to port Sloth or create a block
cipher, only the underlying square root permutation.

1. For a prime size of 256 bits, derive the _largest_ prime and resulting exponent.
2. For a given input block of 256 bits (akin to the plain text), compute the modular square root (akin to the cipher text).
3. For a given output block of 256 btis (akin to the cipher text) compute the inverse square root (which would result in the original plain text).
4. Provide appropriate unit tests and integration tests.
5. Provide appropriate comments and inline documentation
6. Provide an appropriate readme with setup and installation instructions.

_Bonus_

Create benchmarks using criterion for and determine the constant verification speedup for a 256 bit prime.

## Plan:
* Set up project skeleton with criterion benchmarking and unit testing
* Learn how to use the rug crate to create 256 bit blocks
* Translate the equivalent functions from the pysloth C [implementation](https://github.com/randomchain/pysloth/blob/master/sloth.c).

## License
[MIT](https://choosealicense.com/licenses/mit/)