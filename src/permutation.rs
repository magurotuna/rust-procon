// [FROM] https://github.com/bluss/permutohedron
//
//
// Copyright (c) 2015-2017 Ulrik Sverdrup "bluss"
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

/// Permute a slice into its next or previous permutation (in lexical order).
///
/// ```
/// use permutohedron::LexicalPermutation;
///
/// let mut data = [1, 2, 3];
/// let mut permutations = Vec::new();
///
/// loop {
///     permutations.push(data.to_vec());
///     if !data.next_permutation() {
///         break;
///     }
/// }
///
/// assert_eq!(permutations, &[&[1, 2, 3], &[1, 3, 2],
///                            &[2, 1, 3], &[2, 3, 1],
///                            &[3, 1, 2], &[3, 2, 1]]);
///
/// // `data` has been mutated in-place:
/// assert_eq!(data, [3, 2, 1]);
/// ```
pub trait LexicalPermutation {
    /// Return `true` if the slice was permuted, `false` if it is already
    /// at the last ordered permutation.
    fn next_permutation(&mut self) -> bool;
    /// Return `true` if the slice was permuted, `false` if it is already
    /// at the first ordered permutation.
    fn prev_permutation(&mut self) -> bool;
}

impl<T> LexicalPermutation for [T]
where
    T: Ord,
{
    /// Original author in Rust: Thomas Backman <serenity@exscape.org>
    fn next_permutation(&mut self) -> bool {
        // These cases only have 1 permutation each, so we can't do anything.
        if self.len() < 2 {
            return false;
        }

        // Step 1: Identify the longest, rightmost weakly decreasing part of the vector
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] >= self[i] {
            i -= 1;
        }

        // If that is the entire vector, this is the last-ordered permutation.
        if i == 0 {
            return false;
        }

        // Step 2: Find the rightmost element larger than the pivot (i-1)
        let mut j = self.len() - 1;
        while j >= i && self[j] <= self[i - 1] {
            j -= 1;
        }

        // Step 3: Swap that element with the pivot
        self.swap(j, i - 1);

        // Step 4: Reverse the (previously) weakly decreasing part
        self[i..].reverse();

        true
    }

    fn prev_permutation(&mut self) -> bool {
        // These cases only have 1 permutation each, so we can't do anything.
        if self.len() < 2 {
            return false;
        }

        // Step 1: Identify the longest, rightmost weakly increasing part of the vector
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] <= self[i] {
            i -= 1;
        }

        // If that is the entire vector, this is the first-ordered permutation.
        if i == 0 {
            return false;
        }

        // Step 2: Reverse the weakly increasing part
        self[i..].reverse();

        // Step 3: Find the rightmost element equal to or bigger than the pivot (i-1)
        let mut j = self.len() - 1;
        while j >= i && self[j - 1] < self[i - 1] {
            j -= 1;
        }

        // Step 4: Swap that element with the pivot
        self.swap(i - 1, j);

        true
    }
}

#[test]
fn lexical() {
    let mut data = [1, 2, 3];
    data.next_permutation();
    assert_eq!(&data, &[1, 3, 2]);
    data.next_permutation();
    assert_eq!(&data, &[2, 1, 3]);
    data.prev_permutation();
    assert_eq!(&data, &[1, 3, 2]);
    data.prev_permutation();
    assert_eq!(&data, &[1, 2, 3]);
    assert!(!data.prev_permutation());
    let mut c = 0;
    while data.next_permutation() {
        c += 1;
    }
    assert_eq!(c, 5);
}

