#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(deprecated)]

use std::cell::{Cell, Ref, RefCell};
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

pub trait Prime {
    fn lower_primes(&self) -> Vec<usize>;
    fn factorize(&self) -> HashMap<usize, usize>;
}

impl Prime for usize {
    /// エラトステネスの篩を用いてself以下の素数を求める
    fn lower_primes(&self) -> Vec<usize> {
        let &this = self;
        let mut v = Vec::new();
        if this < 2 {
            return v;
        }
        let mut deque = (2..(this + 1)).collect::<VecDeque<usize>>();

        let mut p = match deque.pop_front() {
            Some(x) => x,
            None => return v,
        };
        v.push(p);
        while p as f64 <= (this as f64).sqrt() {
            deque = deque.iter().filter(|&&x| x % p != 0).map(|x| *x).collect();
            p = match deque.pop_front() {
                Some(x) => x,
                None => return v,
            };
            v.push(p);
        }
        for n in deque {
            v.push(n);
        }
        v
    }

    /// エラトステネスの篩を用いてselfを素因数分解する
    fn factorize(&self) -> HashMap<usize, usize> {
        let mut ret = HashMap::new();
        let primes = ((*self as f64).sqrt() as usize).lower_primes();

        let mut tmp = *self;
        for prime in primes {
            while tmp % prime == 0 {
                tmp /= prime;
                *ret.entry(prime).or_insert(0) += 1;
            }
        }
        if tmp > 1 {
            *ret.entry(tmp).or_insert(0) += 1;
        }
        ret
    }
}

fn num_over(hash: &HashMap<usize, usize>, base: usize) -> usize {
    hash.iter().filter(|&(_, &y)| y >= base).count()
}

fn main() {
    let n = read!(usize);

    let mut f = HashMap::new();
    for i in 1usize..(n + 1) {
        let tmp = i.factorize();
        for (n, count) in tmp {
            *f.entry(n).or_insert(0) += count;
        }
    }

    // 自然数nを素因数分解したときの素数の指数をa1, a2, ..., akとすると
    // nの約数の個数は (a1 + 1) * (a2 + 1) * ... * (ak + 1) である
    // 約数の個数が 75 = 3 ^ 1 * 5 ^ 2 であるということは、a1, a2, a3として
    // (4, 4, 2), (14, 4), (24, 2), (74) というような組み合わせになっていれば約数が75個になる

    let n74 = num_over(&f, 74);
    let n24 = num_over(&f, 24);
    let n14 = num_over(&f, 14);
    let n4 = num_over(&f, 4);
    let n2 = num_over(&f, 2);

    //  (4, 4, 2) となるパターン
    let p1 = ((n4 * (n4 - 1)) / 2) * (n2 - 2);

    // (14, 4) となるパターン
    let p2 = n14 * (n4 - 1);

    // (24, 2) となるパターン
    let p3 = n24 * (n2 - 1);

    // (74) となるパターン
    let p4 = n74;

    println!("{}", p1 + p2 + p3 + p4);
}
