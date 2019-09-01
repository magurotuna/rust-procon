#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(deprecated)]

use std::cell::{Cell, Ref, RefCell, RefMut};
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

fn main() {
    let (n, m) = read!(usize, usize);
    let mut s: Vec<Vec<usize>> = Vec::with_capacity(m);
    for i in 0..m {
        s.push(read![[usize]].into_iter().skip(1).map(|x| x - 1).collect());
    }
    let p: Vec<usize> = read![[usize]];

    let mut ans = 0;
    // bit全探索 スイッチ数は高々10なので計算量的には問題なさそう
    'bits: for bits in 0..2usize.pow(n as u32) {
        let tmp = vec![false; n];
        let status: Vec<bool> = tmp
            .into_iter()
            .enumerate()
            .map(|(index, _)| 1 << index & bits != 0)
            .collect();
        'inner: for i in 0..m {
            let num_on = s[i].iter().filter(|&&x| status[x]).count();
            if num_on % 2 == p[i] {
                continue 'inner;
            } else {
                continue 'bits;
            }
        }
        ans += 1;
    }
    println!("{}", ans);
}
