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
    let a = read![[i64]];

    let mut costs: Vec<i64> = Vec::with_capacity(n + 10);
    // とりあえず取りやめない場合のコストを計算 累積和
    for i in 0..n {
        if i == 0 {
            costs.push(a[0].abs());
        } else {
            let prev = costs[i - 1];
            costs.push((a[i] - a[i - 1]).abs() + prev);
        }
    }
    {
        let prev = costs[n - 1];
        costs.push(a[n - 1].abs() + prev);
    }

    for i in 0..n {
        if i == 0 {
            let range1 = costs[n] - costs[1];
            let x = a[1].abs();
            println!("{}", range1 + x);
        } else if i == n - 1 {
            let range1 = costs[i - 1];
            let x = a[i - 1].abs();
            println!("{}", range1 + x);
        } else {
            let range1 = costs[i - 1];
            let range2 = costs[n] - costs[i + 1];
            let x = (a[i - 1] - a[i + 1]).abs();
            println!("{}", range1 + x + range2);
        }
    }
}
