#![feature(test)]

use core::arch::x86_64::{
    _mm256_loadu_si256,
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
    let achunks = a.chunks_exact(32);
    let bchunks = b.chunks_exact(32);

    let mut offset = 0;

    for (achunk, bchunk) in achunks.zip(bchunks) {
        unsafe {
            // load the 32 bits into a "bag of bits"
            let baga = _mm256_loadu_si256(achunk.as_ptr() as *const _);
            let bagb = _mm256_loadu_si256(bchunk.as_ptr() as *const _);

            // compare each byte, -1 if not equal, 0 if equal
            let eq256 = _mm256_cmpeq_epi8(baga, bagb);

            // retrieve the most significant bit of each byte
            let eq = _mm256_movemask_epi8(eq256) as u32;

            // reverse and counts the number of trailing least significant zeros
            let reversed = !eq;
            let first_non_zero = _tzcnt_u32(reversed);

            offset += first_non_zero as usize;

            // return if these 32 bytes doesn't match entirely
            if first_non_zero != 32 {
                return offset
            }
        }
    }

    // if the remaining string is shorter than 32 bytes
    // therefore we fallback on the dumb version
    let a = &a[offset..];
    let b = &b[offset..];
    let count = dumb_longest_prefix(a, b);

    count + offset
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
