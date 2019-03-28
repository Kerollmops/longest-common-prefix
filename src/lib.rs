#![feature(test)]

use std::cmp;
use core::arch::x86_64::{
    _mm256_lddqu_si256,
    _mm256_movemask_epi8,
    _mm256_cmpeq_epi8,
    _tzcnt_u32,
};

#[inline]
pub fn dumb_longest_prefix(a: &[u8], b: &[u8]) -> usize {
    a.iter().zip(b).take_while(|(a, b)| a == b).count()
}

#[inline]
pub fn longest_prefix(a: &[u8], b: &[u8]) -> usize {
    let iterations = cmp::min(a.len(), b.len()) / 32;
    let mut offset = 0;

    for i in 0..iterations {
        let off = i * 32;

        unsafe {
            // loads the 32 bytes from memory into a "bag of bits"
            let baga = _mm256_lddqu_si256(a.as_ptr().add(off) as *const _);
            let bagb = _mm256_lddqu_si256(b.as_ptr().add(off) as *const _);

            // compare each byte, 0xFF if equal, 0x00 if not
            let eq256 = _mm256_cmpeq_epi8(baga, bagb);

            // retrieve the most significant bit of each byte
            // this reduces the mm256 into an i32,
            // inserting a 1 for each 0xFF and a 0 for each 0x00
            let eq = _mm256_movemask_epi8(eq256) as u32;

            // reverse and counts the number of trailing least significant zeros
            // this informs on the biggest number of matching bytes from the start
            let reversed = !eq;
            let first_non_zero = _tzcnt_u32(reversed);

            offset += first_non_zero as usize;

            // if these 32 bytes doesn't match entirely,
            // stop and return the longest offset reached
            if first_non_zero != 32 {
                return offset
            }
        }
    }

    unsafe {
        let a = a.get_unchecked(offset..);
        let b = b.get_unchecked(offset..);

        offset + dumb_longest_prefix(a, b)
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use self::test::Bencher;

    use super::*;

    const STRING_A: &[u8] = b"hello world is a trivial exercise";
    const STRING_B: &[u8] = b"hello world is a trivial example";

    const LITTLE_STRING_A: &[u8] = b"trivial exercise";
    const LITTLE_STRING_B: &[u8] = b"trivial example";

    #[bench]
    fn dumb_version(bench: &mut Bencher) {
        bench.iter(|| {
            let a = test::black_box(STRING_A);
            let b = test::black_box(STRING_B);
            assert_eq!(dumb_longest_prefix(a, b), 27);
        });
    }

    #[bench]
    fn simd_version(bench: &mut Bencher) {
        bench.iter(|| {
            let a = test::black_box(STRING_A);
            let b = test::black_box(STRING_B);
            assert_eq!(longest_prefix(a, b), 27);
        });
    }

    #[bench]
    fn dumb_version_little(bench: &mut Bencher) {
        bench.iter(|| {
            let a = test::black_box(LITTLE_STRING_A);
            let b = test::black_box(LITTLE_STRING_B);
            assert_eq!(dumb_longest_prefix(a, b), 10);
        });
    }

    #[bench]
    fn simd_version_little(bench: &mut Bencher) {
        bench.iter(|| {
            let a = test::black_box(LITTLE_STRING_A);
            let b = test::black_box(LITTLE_STRING_B);
            assert_eq!(longest_prefix(a, b), 10);
        });
    }
}
