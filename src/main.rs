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
    let (w, h, n) = read!(usize, usize, usize);
    let xya: Vec<(usize, usize, usize)> = read!(usize, usize, usize; n);

    let mut p1 = (0usize, 0usize);
    let mut p2: (usize, usize) = (w, h);

    for (x, y, a) in xya {
        if a == 1 {
            p1 = (max(p1.0, x), p1.1);
        } else if a == 2 {
            p2 = (min(p2.0, x), p2.1);
        } else if a == 3 {
            p1 = (p1.0, max(p1.1, y));
        } else if a == 4 {
            p2 = (p2.0, min(p2.1, y));
        }
    }

    if p2.0 > p1.0 && p2.1 > p1.1 {
        println!("{}", (p2.0 - p1.0) * (p2.1 - p1.1));
    } else {
        println!("0");
    }
}
