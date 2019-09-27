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

fn warchall_floyd(vertex: usize, cost: &mut Vec<Vec<usize>>) {
    for k in 0..vertex {
        for j in 0..vertex {
            for i in 0..vertex {
                let ij = cost[i][j];
                let ik = cost[i][k];
                let kj = cost[k][j];
                cost[i][j] = min(ij, ik + kj);
            }
        }
    }
}

fn main() {
    let (h, w) = read!(usize, usize);
    let mut c = vec![];
    for i in 0..10 {
        let t: Vec<usize> = read![[usize]];
        c.push(t);
    }
    let mut a = Vec::with_capacity(h);
    for i in 0..h {
        let t: Vec<i32> = read![[i32]];
        a.push(t);
    }

    warchall_floyd(10, &mut c);

    let mut cost = 0;

    for i in 0..h {
        for j in 0..w {
            if a[i][j] == -1 || a[i][j] == 1 {
                continue;
            }
            cost += c[a[i][j] as usize][1];
        }
    }

    println!("{}", cost);
}
