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
    let s: String = read!(String);

    if &s == "Sunny" {
        println!("Cloudy");
        return;
    }
    if &s == "Cloudy" {
        println!("Rainy");
        return;
    }
    if &s == "Rainy" {
        println!("Sunny");
        return;
    }
}

fn b() {
    let s: String = read!(String);
    let c: Vec<char> = s.chars().collect();

    for i in 0..c.len() {
        let j = i + 1;
        if j % 2 == 0 {
            if c[i] == 'R' {
                println!("No");
                return;
            }
        } else {
            if c[i] == 'L' {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}

fn c() {
    let (n, k, q): (usize, i64, usize) = read!(usize, i64, usize);
    let a: Vec<usize> = read!(usize; q);

    let mut points = vec![k; n]; // 初期ポイント

    let mut resolves = vec![0; n]; // 正解数
    for v in a {
        resolves[v - 1] += 1;
    }

    // 累積和
    let mut s = vec![0; n + 1];
    for i in 1..(n + 1) {
        s[i] = s[i - 1] + resolves[i - 1];
    }

    // ポイント計算
    for i in 0..n {
        // iさん以外の正解数
        let another = s[i] + s[n] - s[i + 1];
        points[i] -= another;
        if points[i] > 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn d() {
    let (n, m) = read!(usize, usize);
    let a: Vec<usize> = read![[usize]];

    let mut bh = BinaryHeap::with_capacity(n);
    for &v in &a {
        bh.push(v);
    }

    for i in 0..m {
        let p = bh.pop().unwrap();
        bh.push(p / 2);
    }

    let mut ans = 0;
    for &v in &bh {
        ans += v;
    }
    println!("{}", ans);
}

fn e() {
    let n = read!(usize);
    let s: String = read!(String);
    let c: Vec<char> = s.chars().collect();

    let mut ans = 0;

    for i in 1..n {
        let left = &c[0..i];
        let right = &c[i..n];

        let mut dp = vec![vec![0usize; right.len()]; left.len()];

        let mut lcs = 0;

        for x in 0..left.len() {
            if left[x] == right[0] {
                dp[x][0] = 1;
                lcs = 1;
            }
        }
        for y in 0..right.len() {
            if right[y] == left[0] {
                dp[0][y] = 1;
                lcs = 1;
            }
        }

        for x in 1..left.len() {
            for y in 1..right.len() {
                dp[x][y] = if left[x] == right[y] {
                    dp[x - 1][y - 1] + 1
                } else {
                    0
                };
                lcs = max(lcs, dp[x][y]);
            }
        }

        ans = max(ans, lcs);
    }

    println!("{}", ans);
}

fn f() {
    unimplemented!();
}

fn main() {
    e();
}
