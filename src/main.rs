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
    let mut f: Vec<Vec<usize>> = Vec::with_capacity(n);
    for i in 0..n {
        f.push(read![[usize]]);
    }
    let mut p: Vec<Vec<i32>> = Vec::with_capacity(n);
    for i in 0..n {
        p.push(read![[i32]]);
    }
    //    let f = read!(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize; n);
    //    let p = read!(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32; n);

    let mut ans = -1_000_000_000;

    // bit全探索
    for o in 1..2i32.pow(10) {
        let mut c = vec![0; n]; // 商店街の店xと同時に開店している時間帯の個数
        for i in 0..10 {
            if 1 << i & o != 0 {
                // 店jがオープンしているか
                for j in 0..n {
                    if f[j][i] == 1 {
                        c[j] += 1;
                    }
                }
            }
        }
        let mut t_sum = 0;
        for i in 0..n {
            t_sum += p[i][c[i]];
        }
        if t_sum > ans {
            ans = t_sum;
        }
    }
    println!("{}", ans);
}
