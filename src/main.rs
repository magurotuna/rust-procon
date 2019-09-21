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

fn a() {
    let (h, w, a, b) = read!(usize, usize, usize, usize);

    let mut left1 = Vec::new();
    let mut right1 = Vec::new();
    for i in 1..(w + 1) {
        if i <= a {
            left1.push("1");
            right1.push("0");
        } else {
            left1.push("0");
            right1.push("1");
        }
    }

    //    let mut ans = Vec::new();
    //
    let sl = left1.join("");
    let sr = right1.join("");
    for i in 1..(h + 1) {
        if i <= b {
            //            ans.push(sl.chars().collect::<Vec<char>>());
            println!("{}", &sl);
        } else {
            //            ans.push(sr.chars().collect::<Vec<char>>());
            println!("{}", &sr);
        }
    }
}

fn b() {
    let (n, k) = read!(usize, usize);
    let p: Vec<usize> = read![[usize]];

    // 連続k要素の選び方はn+1-k通りなので答えは高々n+1-k
    // dp[i][j] := i番目からj番目までが昇順に並んでいるか否か
    let mut dp = vec![vec![false; n]; n];
    for i in 0..n {
        dp[i][i] = true;
    }
    let mut start = 0;
    for x in 1..n {
        if p[x] > p[x - 1] {
            for pos in start..x {
                dp[pos][x] = true;
            }
        } else {
            start = x;
        }
    }

    for i in 0..n {}
}

fn c() {
    unimplemented!();
}

fn d() {
    unimplemented!();
}

fn e() {
    unimplemented!();
}

fn f() {
    unimplemented!();
}

fn main() {
    b();
}
