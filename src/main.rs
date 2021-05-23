mod sloth;






fn main() {
    println!("Hello, world!");
}


// creating this submodule means we won't compile testing code
// when we compile "production-ready" binaries
#[cfg(test)]
mod test {
    // import all code outside of this submodule
    use super::*;
    use rug::{Integer, Assign};

    #[test]
    fn test_basic() {
        let mut a = Integer::new();
        let mut b = Integer::new();
        b.assign(10);
        sloth::dummy(&mut a, b);
        assert_eq!(a, 10);
    }
}