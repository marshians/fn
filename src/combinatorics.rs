//! # Combinatorics
//!
//! `combinatorics` is a collection of
//! [combinatoric](https://en.wikipedia.org/wiki/Combinatorics)
//! utilities. It is not currently full featured as I'm only adding
//! functionality that I need. Pull requests are always welcome though!

use std::iter::Iterator;

/// A representation of a permutation.
///
/// # Example
///
/// ```
/// # use marshians_fn::combinatorics::Permutation;
/// let mut pp = Vec::new();
/// for p in Permutation::new(3) {
///     pp.push(p);
/// }
///
/// assert_eq!(
///     pp,
///     vec![
///         vec![0, 1, 2],
///         vec![1, 0, 2],
///         vec![2, 0, 1],
///         vec![0, 2, 1],
///         vec![1, 2, 0],
///         vec![2, 1, 0]
///     ]
/// );
/// ```
///
/// # Index Based
///
/// The iterator returns zero-based values of the positions in the
/// permutation.  This makes it more generally useful for anything
/// that is indexable.
///
/// Consider permuting a set of letters:
///
/// ```
/// # use marshians_fn::combinatorics::Permutation;
/// let letters = vec!['a', 'b', 'c'];
/// let mut pp = Vec::new();
/// for p in Permutation::new(3) {
///     pp.push(vec![letters[p[0]], letters[p[1]], letters[p[2]]]);
/// }
///
/// assert_eq!(
///     pp,
///     vec![
///         vec!['a', 'b', 'c'],
///         vec!['b', 'a', 'c'],
///         vec!['c', 'a', 'b'],
///         vec!['a', 'c', 'b'],
///         vec!['b', 'c', 'a'],
///         vec!['c', 'b', 'a']
///     ]
/// );
/// ```
#[derive(Clone, Debug)]
pub struct Permutation {
    v: Vec<usize>,
    c: Vec<usize>,
    i: usize,
    n: usize,
    first: bool,
}

impl Permutation {
    /// Create a new permutation iterator for an n-size vector.
    pub fn new(n: usize) -> Permutation {
        let mut v = Vec::new();
        let mut c = Vec::new();
        for x in 0..n {
            v.push(x);
            c.push(0);
        }
        Permutation {
            v,
            c,
            i: 0,
            n,
            first: true,
        }
    }
}

impl Iterator for Permutation {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Vec<usize>> {
        // Heap's Algorithm (https://en.wikipedia.org/wiki/Heap%27s_algorithm)
        if self.first {
            // The initial vector is the first permutation.
            self.first = false;
            return Some(self.v.clone());
        }

        while self.i < self.n {
            if self.c[self.i] < self.i {
                if self.i % 2 == 0 {
                    self.v.swap(0, self.i);
                } else {
                    self.v.swap(self.c[self.i], self.i);
                }
                self.c[self.i] += 1;
                self.i = 0;
                return Some(self.v.clone());
            } else {
                self.c[self.i] = 0;
                self.i += 1;
            }
        }

        None
    }
}

/// A representation of a combination.
///
/// # Example
///
/// ```
/// # use marshians_fn::combinatorics::Combination;
/// let mut cc = Vec::new();
/// for c in Combination::new(3, 2) {
///     cc.push(c);
/// }
///
/// assert_eq!(
///     cc,
///     vec![
///         vec![0, 1],
///         vec![1, 2],
///         vec![0, 2]
///     ]
/// );
/// ```
///
/// # Index Based
///
/// The iterator returns zero-based values of the positions in the
/// combination.  This makes it more generally useful for anything
/// that is indexable.
///
/// Consider combinations of a set of letters:
///
/// ```
/// # use marshians_fn::combinatorics::Combination;
/// let letters = vec!['a', 'b', 'c'];
/// let mut cc = Vec::new();
/// for c in Combination::new(3, 2) {
///     cc.push(vec![letters[c[0]], letters[c[1]]]);
/// }
///
/// assert_eq!(
///     cc,
///     vec![
///         vec!['a', 'b'],
///         vec!['b', 'c'],
///         vec!['a', 'c']
///     ]
/// );
/// ```
#[derive(Clone, Debug)]
pub struct Combination {
    n: usize,
    k: usize,
    i: usize,
}

impl Combination {
    /// Create a new combination iterator for an n-size array where k
    /// elements are taken from it each iteration.
    pub fn new(n: usize, k: usize) -> Combination {
        Combination { n, k, i: 0 }
    }
}

fn gray_code(n: usize) -> usize {
    return n ^ (n >> 1);
}

fn bits(n: usize) -> usize {
    let mut r = 0;
    let mut n = n;
    while n > 0 {
        r += n & 1;
        n >>= 1;
    }
    r
}

impl Iterator for Combination {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Vec<usize>> {
        // We use gray code to subtly modify a bitset:
        // https://en.wikipedia.org/wiki/Gray_code
        while self.i < (1 << self.n) {
            let cur = gray_code(self.i);
            self.i += 1;
            // We return any bitset that matches the request k size.
            if bits(cur) == self.k {
                let mut v = Vec::new();
                for j in 0..self.n {
                    if cur & (1 << j) != 0 {
                        v.push(j)
                    }
                }
                return Some(v);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permutation_3() {
        let mut pp = Vec::new();
        for p in Permutation::new(3) {
            pp.push(p);
        }
        assert_eq!(
            pp,
            vec![
                vec![0, 1, 2],
                vec![1, 0, 2],
                vec![2, 0, 1],
                vec![0, 2, 1],
                vec![1, 2, 0],
                vec![2, 1, 0]
            ]
        );
    }

    #[test]
    fn combination_3_2() {
        let mut cc = Vec::new();
        for c in Combination::new(3, 2) {
            cc.push(c);
        }
        assert_eq!(cc, vec![vec![0, 1], vec![1, 2], vec![0, 2]]);
    }
}
