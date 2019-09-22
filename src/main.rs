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
    let n: usize = read!(usize);
    let mut a: Vec<usize> = read![[usize]];

    a.sort();
    let mut s = Vec::with_capacity(n);
    s.push(a[0]);
    for i in 1..n {
        let prev_s = s[i - 1];
        s.push(prev_s + a[i]);
    }

    let mut a_rev = a.clone();
    a_rev.reverse();

    let mut ans = 1; // 一番大きい大きさをもつ生物は必ず最後まで残ることが可能

    for i in 1..n {
        // i番目(0-based)に大きい生物が最後まで残る可能性があるのは、この生物よりも小さい生物を全部合体した状況で、自分より1つ大きいやつを食えるかどうかで決まる
        if i == n - 1 {
            if a_rev[i] * 2 >= a_rev[i - 1] {
                ans += 1;
            }
        } else if (a_rev[i] + s[n - i - 2]) * 2 >= a_rev[i - 1] {
            ans += 1;
        } else {
            break;
        }
    }
    println!("{}", ans);
}
