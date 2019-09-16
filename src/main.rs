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
    let (x, y) = read!(i64, i64);

    if x == y {
        println!("0");
        return;
    }

    if x * y >= 0 {
        // x, yの符号が同じ場合（どちらかが0である場合も含む）
        if x < y {
            println!("{}", y - x);
            return;
        } else {
            println!("{}", x - y + 2);
            return;
        }
    } else {
        // x, yの符号が違う場合
        if x > 0 {
            // xが正、yが負
            // |x| と |y| の大小で場合分けが生じる
            if x.abs() >= y.abs() {
                println!("{}", 1 + x + y);
                return;
            } else {
                println!("{}", 1 - x - y);
                return;
            }
        } else {
            // xが負、yが正
            if x.abs() >= y.abs() {
                println!("{}", 1 - x - y);
                return;
            } else {
                println!("{}", 1 + x + y);
                return;
            }
        }
    }
}
