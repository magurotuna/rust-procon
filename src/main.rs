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
    let (n, k): (usize, usize) = read!(usize, usize);
    let a: Vec<usize> = read![[usize]];

    if k == n {
        println!("1");
        return;
    }

    println!("{}", (((n - 1) as f64) / ((k - 1) as f64)).ceil())

    // 以下 k < n とする

    //    let one_pos = a.iter().position(|&x| x == 1).unwrap();
    //    let mut one_range = (one_pos, one_pos + 1);

    // 1を区間の端に含むような幅kの区間を選んでいけば最小になる
    // ↑ウソ！！！！！！
    //
    //    let mut ans = 0;
    //
    //    while one_range.0 != 0 || one_range.1 != n {
    //        let left_len = match one_range.0.checked_sub(k - 1) {
    //            Some(v) => k,
    //            None => one_range.0 + 1,
    //        };
    //        let right_len = match one_range.1 - 1 + k {
    //            v if v > n => n - (one_range.1 - 1),
    //            _ => k,
    //        };
    //        if left_len >= right_len {
    //            one_range.0 = one_range.0 + 1 - left_len;
    //        } else {
    //            one_range.1 = one_range.1 + right_len - 1;
    //        }
    //        ans += 1;
    //    }
    //    println!("{}", ans)
}
