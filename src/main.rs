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
    let (n, m): (usize, usize) = read!(usize, usize);

    // 赤ちゃんの人数zを固定した上でxとyについて鶴亀算をする
    for z in 0..(n + 1) {
        // 赤ちゃんの足の本数は4z
        // 2x + 3y = M - 4z かつ x + y = N - z
        // を満たすような(x, y)を求める
        let leg_s = m - 4 * z;
        let num_s = n - z;

        // 仮に (x, y) = (N - z, 0) であるとする
        let leg_tmp = 2 * num_s;
        let diff = leg_s as i64 - leg_tmp as i64;
        // xを1減らしyを1増やすと足の本数は1つ増えるということに着目
        if 0 <= diff && diff <= num_s as i64 {
            let x = num_s as i64 - diff;
            let y = diff;
            println!("{} {} {}", x, y, z);
            return;
        }
    }

    println!("-1 -1 -1");
}
