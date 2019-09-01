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

fn a() {
    let s = read!(String);
    let t = read!(String);

    let c_s: Vec<char> = s.chars().collect::<Vec<char>>();
    let t_s: Vec<char> = t.chars().collect::<Vec<char>>();
    let mut ans = 0;
    for i in 0..c_s.len() {
        if c_s[i] == t_s[i] {
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn b() {
    let (a, b) = read!(usize, usize);
    if b == 1 {
        println!("0");
        return;
    }
    let mut ans = 1;
    let mut cnt = a;
    while cnt < b {
        ans += 1;
        cnt += a - 1;
    }
    println!("{}", ans);
}

fn c() {
    let n = read!(usize);
    let h: Vec<usize> = read![[usize]];

    // 最長部分減少列（ただし連続）の長さを求める その長さ-1が答え
    let mut max_len = 0;
    let mut len = 0;
    let mut prev_h = 0;
    for i in 0..n {
        if prev_h >= h[i] {
            prev_h = h[i];
            len += 1;
        } else {
            prev_h = h[i];
            if max_len < len {
                max_len = len;
            }
            len = 1;
        }
    }
    if max_len < len {
        max_len = len;
    }
    println!("{}", max_len - 1);
}

fn d() {
    let n = read!(usize);

    let ans = (n * (n - 1)) / 2;
    println!("{}", ans);
}

fn e() {
    unimplemented!();
}

fn f() {
    unimplemented!();
}

fn main() {
    a();
}
