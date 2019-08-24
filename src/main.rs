#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
use std::cell::RefCell;
use std::cmp::{max, min, Ordering};
use std::collections::*;
use std::fmt::{Debug, Formatter, Write as FmtWrite};
use std::io::{stderr, stdin, BufRead, Write};
use std::mem::{replace, swap};
use std::ops::*;
use std::rc::Rc;
use std::usize;

const MOD_10_9_7: u64 = 1_000_000_007;

/// FYI: https://github.com/vain0x/scan-bench
#[allow(unused_macros)]
macro_rules! read {
    ([$t:ty] ; $n:expr) =>
        ((0..$n).map(|_| read!([$t])).collect::<Vec<_>>());
    ($($t:ty),+ ; $n:expr) =>
        ((0..$n).map(|_| read!($($t),+)).collect::<Vec<_>>());
    ([$t:ty]) =>
        (rl().split_whitespace().map(|w| w.parse().unwrap()).collect::<Vec<$t>>());
    ($t:ty) =>
        (rl().parse::<$t>().unwrap());
    ($($t:ty),*) => {{
        let buf = rl();
        let mut w = buf.split_whitespace();
        ($(w.next().unwrap().parse::<$t>().unwrap()),*)
    }};
}

fn rl() -> String {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim_right().to_owned()
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// 飛ばし飛ばしで値を使うイテレータ Rust v1.28以降でしか使えないため独自実装
/// 簡易実装のためインデックスのオーバーフロー等は考慮していない
pub struct StepBy<I> {
    iter: I,
    step: usize,
    first_take: bool,
}
impl<I> StepBy<I> {
    pub fn new(iter: I, step: usize) -> StepBy<I> {
        StepBy {
            iter: iter,
            step: step - 1,
            first_take: true,
        }
    }
}
impl<I> Iterator for StepBy<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_take {
            self.first_take = false;
            self.iter.next()
        } else {
            self.iter.nth(self.step)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let inner_hint = self.iter.size_hint();
        if self.first_take {
            let f = |n| {
                if n == 0 {
                    0
                } else {
                    1 + (n - 1) / (self.step + 1)
                }
            };
            (f(inner_hint.0), inner_hint.1.map(f))
        } else {
            let f = |n| n / (self.step + 1);
            (f(inner_hint.0), inner_hint.1.map(f))
        }
    }
}
impl<I> ExactSizeIterator for StepBy<I> where I: ExactSizeIterator {}
////////////////////////////////////////////////////////////////////////////////////////////////////

trait IteratorExt: Iterator + Sized {
    // ref: https://qiita.com/vain0x/items/512784ff60ce599dccae#vec
    fn vec(self) -> Vec<Self::Item> {
        self.collect()
    }

    // step_by は v1.28.0 以降でしか使えないため独自実装
    fn stepby(self, step: usize) -> StepBy<Self>
    where
        Self: Sized,
    {
        StepBy::new(self, step)
    }
}

impl<T: Iterator> IteratorExt for T {}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// 逆順ソート ReverseがRust v1.19以降でしか使えないため独自実装
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
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
// 逆順ソートここまで
////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
/// べき乗のmodを効率的に求める関数
/// ref: https://algorithmbeginner.blogspot.com/2018/02/blog-post_23.html
fn pow(x: u64, n: u64, modulo: u64) -> u64 {
    let mut res = 1;
    if n > 0 {
        res = pow(x, n / 2, modulo);
        if n % 2 == 0 {
            res = (res * res) % modulo;
        } else {
            res = (((res * res) % modulo) * x) % modulo;
        }
    }
    res
}
////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    let n: i64 = read!(i64);
    let v: Vec<usize> = read![[usize]];

    let half_n = n / 2;

    // 1種類の数からなる数列の場合
    if v.iter().collect::<HashSet<&usize>>().len() == 1 {
        println!("{}", &half_n);
        return;
    }

    let mut h_odd = HashMap::new();
    let mut h_even = HashMap::new();

    for i in 0..v.len() {
        if i % 2 == 0 {
            *h_even.entry(v[i]).or_insert(0) += 1;
        } else {
            *h_odd.entry(v[i]).or_insert(0) += 1;
        }
    }

    let mut v_odd: Vec<(usize, usize)> = h_odd.iter().map(|(&key, &value)| (key, value)).collect();
    v_odd.sort_by_key(|&(_a, b)| Rev(b));

    let mut v_even: Vec<(usize, usize)> =
        h_even.iter().map(|(&key, &value)| (key, value)).collect();
    v_even.sort_by_key(|&(_a, b)| Rev(b));

    let (odd_max_digit, odd_max_count) = v_odd[0];
    let (even_max_digit, even_max_count) = v_even[0];

    let odd_change_count = (half_n - odd_max_count as i64).abs();
    let even_change_count = (half_n - even_max_count as i64).abs();

    if odd_max_digit != even_max_digit {
        println!("{}", odd_change_count + even_change_count);
        return;
    } else {
        // 一番出現回数が多い文字がoddとevenで同じ場合、
        // (1) oddの2番目に出現回数が多い文字への書き換え + even_change_count
        // (2) evenの2番めに出現回数が多い文字への書き換え + odd_change_count
        // の両方を試してみて、書き換え回数が小さい方が答えとなる
        let count1 = (half_n - v_odd[1].1 as i64).abs() + even_change_count;
        let count2 = (half_n - v_even[1].1 as i64).abs() + odd_change_count;
        println!("{}", min(count1, count2));
    }
}
