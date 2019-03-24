#![feature(test)]

pub fn longest_prefix(a: &[u8], b: &[u8]) -> usize {
    a.iter().zip(b).take_while(|(a, b)| a == b).count()
}

#[cfg(test)]
mod bench {
    extern crate test;
    use self::test::Bencher;

    use super::*;

    #[bench]
    fn easy(bench: &mut Bencher) {
        let a = b"hello world is a trivial exercise";
        let b = b"hello world is a trivial example";

        bench.iter(|| {
            assert_eq!(longest_prefix(a, b), 27);
        });
    }
}
