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

    if n == 1 {
        println!("{}", m);
        return;
    }

    // a1, a2, ..., aNの公約数をDとすると、SUM(a)もDを約数にもつ。
    // よってMの正の約数のうちMではないものを大きい順から m1, m2, ..., 1として、
    // 順に「Mをm_iで割った余りが0 かつ 商がN以上」であるかどうかを検証していく
    // この条件を満たす最初のm_iが答えである

    let mut d = Vec::new(); // Mの約数リスト
    let root_m = (m as f64).sqrt();
    for i in 1..(root_m.floor() as usize) {
        if m % i == 0 {
            let q = m / i;
            d.push(q);
            d.push(i);
        }
    }
    d.sort();
    d.reverse();

    let d_without_m = &d[1..];

    for &e in d_without_m {
        if m % e == 0 && m / e >= n {
            println!("{}", e);
            return;
        }
    }
}
