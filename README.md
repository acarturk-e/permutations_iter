# permutations_iter

An iterative permutation generator _without recursion_ for Rust.

Iterator `Permutations::of(n)` generates permutations of `0..n` iteratively using
Steinhaus-Johnson-Trotter algorithm with Even's modification.

Each `next()` call has $O(n)$ time and space complexity.

Not optimized. At all. Any improvements are welcome.

Published under MIT license.
