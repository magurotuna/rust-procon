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

fn main() {
    let n = read!(usize);
    let A = read![[i64]];

    // 動的計画法
    // 左からi本目(0-based)の柱にいくための最小コストをdp[i]とする
    let mut dp = vec![0i64; n];

    dp[0] = 0;
    dp[1] = (A[1] - A[0]).abs();

    for i in 2..n {
        let from_prev = (A[i] - A[i - 1]).abs();
        let from_prev2 = (A[i] - A[i - 2]).abs();
        dp[i] = if dp[i - 2] + from_prev2 < dp[i - 1] + from_prev {
            dp[i - 2] + from_prev2
        } else {
            dp[i - 1] + from_prev
        };
    }

    println!("{}", &dp[n - 1]);
}
