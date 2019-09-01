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
    let (n, m) = read!(usize, usize);

    // DPでやってみる
    // dp[i][j] := 人間の数i, 足の数jに対して存在しうる大人、老人、赤ちゃんの人数x, y, zの組合せ (x, y, z)
    // と定義
    let mut dp: Vec<Vec<Option<(usize, usize, usize)>>> = vec![vec![None; m + 100]; n + 100];
    dp[0][1] = Some((1, 0, 0));
    dp[0][2] = Some((0, 1, 0));
    dp[0][3] = Some((0, 0, 1));

    for i in 1..n {
        for j in (2 * i + 1)..(4 * i + 4) {
            'k: for k in 2..5 {
                let mut update_flag = false;
                dp[i][j] = match dp[i - 1][j - k] {
                    Some((x, y, z)) => {
                        update_flag = true;
                        if k == 2 {
                            Some((x + 1, y, z))
                        } else if k == 3 {
                            Some((x, y + 1, z))
                        } else {
                            Some((x, y, z + 1))
                        }
                    }
                    None => None,
                };
                if update_flag {
                    break 'k;
                }
            }
        }
    }

    let (x, y, z) = match dp[n - 1][m - 1] {
        Some(t) => (t.0 as i64, t.1 as i64, t.2 as i64),
        None => (-1, -1, -1),
    };
    println!("{} {} {}", x, y, z);
}
