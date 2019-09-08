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
    let n = read!(usize);

    // dp[i] := i円を実現可能な最小の操作回数 とする
    let mut dp: Vec<usize> = vec![INF as usize; n + 1];

    dp[0] = 0; // 0円
    dp[1] = 1; // 1円
    for i in 2..(n + 1) {
        let mut count = i; // 最悪でも1円ずつi回引き出せば実現できる

        let mut six_index = 1;
        let mut nine_index = 1;
        while i >= 6usize.pow(six_index) {
            let tmp_cnt = dp[i - 6usize.pow(six_index)] + 1;
            if tmp_cnt < count {
                count = tmp_cnt;
            }
            six_index += 1;
        }
        while i >= 9usize.pow(nine_index) {
            let tmp_cnt = dp[i - 9usize.pow(nine_index)] + 1;
            if tmp_cnt < count {
                count = tmp_cnt;
            }
            nine_index += 1;
        }

        dp[i] = count;
    }

    println!("{}", dp[n]);
}
