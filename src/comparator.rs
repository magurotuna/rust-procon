#![allow(dead_code)]

use std::cmp::Ordering;

/// 逆順ソート .rev() がRust v1.19 以降でしか使えないため独自実装
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Rev<T>(pub T);

impl<T: PartialOrd> PartialOrd for Rev<T> {
    fn partial_cmp(&self, other: &Rev<T>) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<T: Ord> Ord for Rev<T> {
    fn cmp(&self, other: &Rev<T>) -> Ordering {
        other.0.cmp(&self.0)
    }
}

pub trait DescSort<T> {
    fn desc_sort(&mut self);
}

impl<T: Ord + PartialOrd + Copy> DescSort<T> for [T] {
    fn desc_sort(&mut self) {
        self.sort_by_key(|&x| Rev(x));
    }
}

/********** 以下テスト **********/

#[test]
fn test_rev() {
    let mut c: Vec<char> = "badc".chars().collect();
    c.sort_by_key(|&x| Rev(x));
    assert_eq!(vec!['d', 'c', 'b', 'a'], c);

    let mut d = vec![1usize, 2, 3, 4];
    d.desc_sort();
    assert_eq!(vec![4usize, 3, 2, 1], d);
}
