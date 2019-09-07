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
    let n = read!(u64);

    // nが 2^(d-1)以上 2^d 未満の場合、nの深さがdである、ということにする
    let mut d = 1;
    loop {
        if 2u64.pow(d - 1) <= n && n < 2u64.pow(d) {
            break;
        }
        d += 1;
    }

    let mut x = 1u64;
    if d % 2 == 0 {
        // Takahashiは2xを、Aokiは2x+1を選ぶのが最善である場合
        loop {
            // Takahashiの操作
            x = 2 * x;
            if x > n {
                println!("Aoki");
                return;
            }

            // Aokiの操作
            x = 2 * x + 1;
            if x > n {
                println!("Takahashi");
                return;
            }
        }
    } else {
        // Takahashiは2x+1を、Aokiは2xを選ぶのが最善である場合
        loop {
            // Takahashiの操作
            x = 2 * x + 1;
            if x > n {
                println!("Aoki");
                return;
            }

            // Aokiの操作
            x = 2 * x;
            if x > n {
                println!("Takahashi");
                return;
            }
        }
    }
}
