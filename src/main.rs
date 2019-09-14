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
    let a: Vec<i64> = read![[i64]];

    let mut ans_even = 0;
    let mut ans_odd = 0;
    let mut s_even = 0;
    let mut s_odd = 0;

    // 偶数番目を正にする場合
    let mut even = true;
    for &ai in &a {
        let tmp = s_even + ai;
        if even {
            // 偶数
            if tmp > 0 {
                s_even = tmp;
            } else {
                ans_even += 1 - tmp;
                s_even = 1;
            }
        } else {
            // 奇数
            if tmp < 0 {
                s_even = tmp;
            } else {
                ans_even += 1 + tmp;
                s_even = -1;
            }
        }
        even = !even;
    }

    // 奇数番目を正にする場合
    let mut odd = false;
    for &ai in &a {
        let tmp = s_odd + ai;
        if odd {
            // 奇数
            if tmp > 0 {
                s_odd = tmp;
            } else {
                ans_odd += 1 - tmp;
                s_odd = 1;
            }
        } else {
            // 偶数
            if tmp < 0 {
                s_odd = tmp;
            } else {
                ans_odd += 1 + tmp;
                s_odd = -1;
            }
        }
        odd = !odd;
    }

    println!("{}", min(ans_even, ans_odd));
}
