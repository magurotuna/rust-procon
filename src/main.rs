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
    let (n, c) = read!(usize, usize);
    let mut d: Vec<Vec<usize>> = vec![];
    for i in 0..c {
        let t: Vec<usize> = read![[usize]];
        d.push(t);
    }
    let mut color = vec![];
    for i in 0..n {
        let t: Vec<usize> = read![[usize]];
        color.push(t);
    }

    // 行と列の和をkとする。kを3で割った余りでグループ分けし、グループ0, 1, 2とする。
    // 各グループをどの色で塗ることにすれば一番違和感が小さくなるのかを考える。

    // グループ0の塗り方c0, 以下順にc1, c2とする
    let mut min_diff = INF as usize;

    // g0, g1, g2 それぞれで元の色iが何個あるのかをあらかじめ数えておくことで計算量減らせないかな？
    let mut g0_map = HashMap::new();
    let mut g1_map = HashMap::new();
    let mut g2_map = HashMap::new();
    for i in 0..n {
        for k in 0..n {
            let ii = i + 1;
            let kk = k + 1;
            match (ii + kk) % 3 {
                0 => {
                    *g0_map.entry(color[i][k] - 1).or_insert(0) += 1;
                }
                1 => {
                    *g1_map.entry(color[i][k] - 1).or_insert(0) += 1;
                }
                2 => {
                    *g2_map.entry(color[i][k] - 1).or_insert(0) += 1;
                }
                _ => (),
            };
        }
    }

    for c0 in 0..c {
        for c1 in 0..c {
            for c2 in 0..c {
                if c0 == c1 || c1 == c2 || c2 == c0 {
                    continue;
                }
                let mut g0_diff = 0;
                let mut g1_diff = 0;
                let mut g2_diff = 0;
                for (&cc, &num) in &g0_map {
                    g0_diff += d[cc][c0] * num;
                }
                for (&cc, &num) in &g1_map {
                    g1_diff += d[cc][c1] * num;
                }
                for (&cc, &num) in &g2_map {
                    g2_diff += d[cc][c2] * num;
                }
                min_diff = min(min_diff, g0_diff + g1_diff + g2_diff);
            }
        }
    }

    println!("{}", min_diff);
}
