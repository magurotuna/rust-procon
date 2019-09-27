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

pub trait CharToNum {
    fn to_i32(&self) -> i32;
    fn to_i64(&self) -> i64;
    fn to_u64(&self) -> u64;
    fn to_usize(&self) -> usize;
}

impl CharToNum for char {
    /// 1, 2, 3, ... の数字の文字型をもとにi32を返す
    fn to_i32(&self) -> i32 {
        *self as i32 - 48
    }

    /// 1, 2, 3, ... の数字の文字型をもとにi64を返す
    fn to_i64(&self) -> i64 {
        *self as i64 - 48
    }

    /// 1, 2, 3, ... の数字の文字型をもとにi64を返す
    fn to_u64(&self) -> u64 {
        *self as u64 - 48
    }

    /// 1, 2, 3, ... の数字の文字型をもとにusizeを返す
    fn to_usize(&self) -> usize {
        *self as usize - 48
    }
}

fn main() {
    let s = read!(String);
    let sc: Vec<char> = s.chars().collect::<Vec<char>>();
    let num = sc.iter().map(|&x| x.to_i32()).collect::<Vec<_>>();

    for i in 0..2usize.pow(3) {
        let mut ops = vec![];
        for j in 0..3 {
            if 1 << j & i != 0 {
                ops.push('+');
            } else {
                ops.push('-');
            }
        }
        let mut s = num[0];
        for k in 0..ops.len() {
            if ops[k] == '+' {
                s += num[k + 1];
            } else {
                s -= num[k + 1];
            }
        }
        if s == 7 {
            println!(
                "{}{}{}{}{}{}{}=7",
                num[0], ops[0], num[1], ops[1], num[2], ops[2], num[3]
            );
            return;
        }
    }
}
