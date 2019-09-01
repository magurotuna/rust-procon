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
    let s = read!(String);
    let k = read!(usize);
    let k0 = k - 1; // 0-based

    // 文字列sを左から見ていって、1以外の数字がはじめて登場するインデックスをi、その1以外の数字をdとする
    // ex: "2345" なら i = 0, d = 2
    // ex: "12345" なら i = 1, d = 2
    // ex: "11111342" なら i = 5, d = 3
    // 5000兆日後のsは、インデックス0からi-1までは「1」、それ以降はdがずっと並ぶ形になる（少なくともk <= 10^18 の範囲においては）
    // このようなiとdを求めて、k0 <= i - 1 ならば1, そうでないならdを出力すればよい

    let c: Vec<char> = s.chars().collect();
    if c.iter().all(|&x| x == '1') {
        println!("1");
        return;
    }
    let i = c.iter().position(|&x| x != '1').unwrap();
    let d = c.get(i).unwrap();
    if k0 >= i {
        println!("{}", d);
    } else {
        println!("1");
    }
}
