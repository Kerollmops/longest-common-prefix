# Longest Common Prefix

Returns the longest common prefix length between two bytes slices.

```
    Finished release [optimized] target(s) in 0.06s
     Running target/release/deps/longest_common_prefix-e79a7221285fbbb4

running 4 tests
test bench::dumb_version        ... bench:          11 ns/iter (+/- 2)
test bench::dumb_version_little ... bench:           7 ns/iter (+/- 2)
test bench::simd_version        ... bench:           8 ns/iter (+/- 1)
test bench::simd_version_little ... bench:           8 ns/iter (+/- 0)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out
```
