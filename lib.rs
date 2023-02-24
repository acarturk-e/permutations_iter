//! Generate permutations iteratively without recursion.
//!
//! ``Permutations.of(n)`` function generates an iterator instance for permutations of `0..n`.
//!
//! ``Permutations.next()`` uses Steinhaus-Johnson-Trotter algorithm with Even's modification to generate the next permutation
//! in $O(n)$ time.
//!
//! Each iterator is one-way. You need to construct a new one for iterating again.

/// Generates the inverse permutation. Has $O(n)$ time complexity.
pub fn inverse_perm(perm: &Vec<usize>) -> Vec<usize> {
    let mut rev_perm = perm.clone();
    for (ii, i) in perm.iter().enumerate() {
        rev_perm[*i] = ii;
    }
    rev_perm
}

/// Implements ``Iterator``.
pub struct Permutations {
    n: usize,
    perm: Vec<usize>,
    direction: Vec<i8>,
    is_initiated: bool,
    is_finished: bool,
}

impl Permutations {
    /// n must be greater than 0!
    pub fn of(n: usize) -> Permutations {
        assert!(n > 0);
        Permutations {
            n,
            perm: vec![0; n],
            direction: vec![0; n],
            is_initiated: false,
            is_finished: false,
        }
    }

    pub fn get_n(&self) -> usize {
        self.n
    }
}

impl Iterator for Permutations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.is_initiated {
            for i in 1..self.n {
                self.perm[i] = i;
                self.direction[i] = -1;
            }
            self.is_initiated = true;
            return Some(self.perm.clone());
        }
        if self.is_finished {
            return None;
        }
        // We know we have a valid perm & direction here, but we may be finished.
        let rev_perm = inverse_perm(&self.perm);
        // Find largest element i that has non-zero direction
        let mut i = self.n;
        let mut ii = 0;
        let mut id = 0;
        for j in (0..self.n).rev() {
            ii = rev_perm[j];
            id = self.direction[ii];
            if id != 0 {
                i = j;
                break;
            }
        }
        if i == self.n {
            self.is_finished = true;
            return None;
        }
        // Swap i along direction i
        let ii_new = (ii as isize + id as isize) as usize;
        self.perm.swap(ii, ii_new);
        self.direction.swap(ii, ii_new);
        // Update directions
        if ii_new == 0
            || ii_new == self.n - 1
            || self.perm[(ii_new as isize + id as isize) as usize] > i
        {
            self.direction[ii_new] = 0;
        }
        for j in (i + 1)..self.n {
            let ji = rev_perm[j];
            self.direction[ji] = if ji < ii_new { 1 } else { -1 };
        }
        Some(self.perm.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::Permutations;

    /// Prints permutations of 4
    #[test]
    fn print_permutations_of_4() {
        println!("All permutations of [4]");
        for perm in Permutations::of(4) {
            println!("{:?}", perm);
        }
    }

    /// Prints first 50 permutations of 20
    #[test]
    fn print_first_50_permutations_of_20() {
        println!("First 50 permutations of [20]");
        for perm in Permutations::of(20).take(50) {
            println!("{:?}", perm);
        }
    }
}
