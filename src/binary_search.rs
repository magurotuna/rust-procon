use std::cmp::Ordering;

/// ref: https://github.com/hatoo/competitive-rust-snippets/blob/master/src/binary_search.rs
pub trait BinarySearch<T> {
    fn lower_bound_index(&self, x: &T) -> usize;
    fn upper_bound_index(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound_index(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }

    fn upper_bound_index(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}

/********** 以下テスト **********/

#[test]
fn test_lower_bound_index() {
    let v = vec![1, 2, 3, 4, 4, 4, 6];
    assert_eq!(v.lower_bound_index(&4), 3);

    let v = vec![1, 1, 1, 1, 1, 1];
    assert_eq!(v.lower_bound_index(&1), 0);

    let v = vec![1, 1, 1, 1, 2, 2];
    assert_eq!(v.lower_bound_index(&2), 4);
}

#[test]
fn test_upper_bound_index() {
    let v = vec![1, 2, 3, 4, 4, 4, 6];
    assert_eq!(v.upper_bound_index(&4), 6);

    let v = vec![1, 1, 1, 1, 1, 1];
    assert_eq!(v.upper_bound_index(&1), 6);

    let v = vec![1, 1, 1, 1, 2, 2];
    assert_eq!(v.upper_bound_index(&2), 6);

    let v = vec![1, 1, 1, 1, 2, 2];
    assert_eq!(v.upper_bound_index(&3), 6);

    let v = vec![1, 1, 1, 1, 2, 2];
    assert_eq!(v.upper_bound_index(&0), 0);
}
