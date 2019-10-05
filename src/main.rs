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
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}

#[allow(unused_macros)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn main() {
    let n = read!(usize);
    let abc: Vec<(usize, usize, usize)> = read!(usize, usize, usize; n);

    // dp[i][j(=0, 1, 2)] := i日目にjという活動をおこなった場合の最大幸福度
    let mut dp = vec![vec![0; 3]; n];

    dp[0][0] = abc[0].0;
    dp[0][1] = abc[0].1;
    dp[0][2] = abc[0].2;

    for i in 1..n {
        dp[i][0] = max(dp[i - 1][1] + abc[i].0, dp[i - 1][2] + abc[i].0);
        dp[i][1] = max(dp[i - 1][0] + abc[i].1, dp[i - 1][2] + abc[i].1);
        dp[i][2] = max(dp[i - 1][1] + abc[i].2, dp[i - 1][0] + abc[i].2);
    }

    let ans = max(dp[n - 1][0], max(dp[n - 1][1], dp[n - 1][2]));
    println!("{}", ans);
}
