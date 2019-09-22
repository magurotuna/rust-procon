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
    let (n, x): (usize, usize) = read!(usize, usize);

    let mut l = Vec::with_capacity(n + 1); // レベルiバーガーの層の数
    let mut p = Vec::with_capacity(n + 1); // レベルiバーガーのパティの数
    l.push(1usize);
    p.push(1usize);
    for i in 1..(n + 1) {
        let l_prev = l[i - 1];
        let p_prev = p[i - 1];
        l.push(l_prev * 2 + 3);
        p.push(p_prev * 2 + 1);
    }

    let mut ans = 0usize;
    let mut i = 0usize;
    let mut i_in_d = 0usize;
    let mut cur_level = n;
    while i < x {
        if i_in_d == 0 {
            // レベルcur_levelバーガーの一番下を見ているとき
            if cur_level == 0 {
                i_in_d = 0;
                i += 1;
                ans += 1;
                continue;
            } else {
                i_in_d += 1;
                i += 1;
                continue;
            }
        }
        if i_in_d == l[cur_level - 1] + 1 {
            // レベルcur_levelバーガーの中央にあるパティを見ているとき
            i_in_d += 1;
            i += 1;
            ans += 1;
            continue;
        }
        if i_in_d == l[cur_level - 1] * 2 + 2 {
            // レベルcur_levelバーガーの一番上を見ているとき
            i_in_d += 1;
            i += 1;
            continue;
        }
        if i + l[cur_level - 1] < x {
            // レベルcur_level - 1バーガーを全部食べても大丈夫なとき
            i_in_d += l[cur_level - 1];
            i += l[cur_level - 1];
            ans += p[cur_level - 1];
            continue;
        }

        // レベルcur_level - 1バーガーを全部食べることができない場合
        i_in_d = 0;
        cur_level -= 1;
    }
    println!("{}", ans);
}
