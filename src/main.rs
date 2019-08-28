#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(deprecated)]

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
const INF: i64 = 1_000_000_000_000;
const MIN_INF: i64 = -1_000_000_000_000;

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

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),*) => {
        eprintln!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}

fn main() {
    let n = read!(usize);
    let a: Vec<usize> = read![[usize]];
    let mut rev_a = a.clone();
    rev_a.reverse();

    let mut ans_vec: Vec<usize> = Vec::with_capacity(n);
    if n % 2 == 0 {
        let first_half = rev_a.into_iter().stepby(2).collect::<Vec<usize>>();
        let latter_half = a.into_iter().stepby(2).collect::<Vec<usize>>();
        ans_vec.extend(first_half);
        ans_vec.extend(latter_half);
    } else {
        let first_half = rev_a.into_iter().stepby(2).collect::<Vec<usize>>();
        let latter_half = a.into_iter().skip(1).stepby(2).collect::<Vec<usize>>();
        ans_vec.extend(first_half);
        ans_vec.extend(latter_half);
    }
    let x = ans_vec
        .iter()
        .map(|&y| y.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", &x);
}
