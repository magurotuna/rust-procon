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
    let (n, q) = read!(usize, usize);
    let s: String = read!(String);
    let lr: Vec<(usize, usize)> = read!(usize, usize; q);

    let lr: Vec<_> = lr.into_iter().map(|x| (x.0 - 1, x.1 - 1)).collect();

    let c: Vec<char> = s.chars().collect();

    // dp[i] := 文字列Sの半開区間 [0, i) に「左隣がAであるようなC」が現れる回数
    let mut dp = vec![0usize; n + 1];
    for i in 2..(n + 1) {
        dp[i] = if c[i - 2] == 'A' && c[i - 1] == 'C' {
            dp[i - 1] + 1
        } else {
            dp[i - 1]
        }
    }

    // lからr（両端含む）部分文字列は区間でいうと [l, r+1)
    // この区間内にある'AC'の個数を数えるには、[l+1, r+1)にある「左隣がAであるようなC」をカウントすればよい
    // つまり dp[r+1] - dp[l+1]
    for &(l, r) in &lr {
        println!("{}", dp[r + 1] - dp[l + 1]);
    }
}
