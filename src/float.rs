#![allow(dead_code)]

use std::cmp::Ordering;

#[derive(PartialEq, PartialOrd)]
pub struct OrdFloat<T>(pub T);

impl<T: PartialEq> Eq for OrdFloat<T> {}

impl<T: PartialOrd> Ord for OrdFloat<T> {
    fn cmp(&self, other: &OrdFloat<T>) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

#[test]
fn test_ord_float() {
    let mut vec = vec![1.3, -1.3, 9.5, 2.5, -1000.2];
    vec.sort_by_key(|&f| OrdFloat(f));

    assert_eq!(vec, vec![-1000.2, -1.3, 1.3, 2.5, 9.5]);
}
