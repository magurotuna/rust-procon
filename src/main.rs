#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(deprecated)]

use std::cell::RefCell;
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

fn main() {
    let (h, w): (i64, i64) = read!(i64, i64);

    let mut ans = INF;
    for i in 1..(w + h - 1) {
        // i <= w - 1 のときは x = i のところで1つ目の区切りを入れる
        if i <= w - 1 {
            let first_s = i * h;

            // 2つ目の区切りは縦方向の中央に入れるか横方向の中央に入れるかの2通り
            // 横方向に区切りを入れる場合
            let second_s_h = (w - i) * (h / 2);
            let third_s_h = h * w - first_s - second_s_h;
            // 縦方向に区切りを入れる場合
            let second_s_v = h * ((w - i) / 2);
            let third_s_v = h * w - first_s - second_s_v;

            let s_diff_h =
                max(first_s, max(second_s_h, third_s_h)) - min(first_s, min(second_s_h, third_s_h));
            let s_diff_v =
                max(first_s, max(second_s_v, third_s_v)) - min(first_s, min(second_s_v, third_s_v));
            let s_diff = min(s_diff_h, s_diff_v);
            ans = if s_diff < ans { s_diff } else { ans };
        } else {
            // i >= w のときは y = i - w + 1 のところで1つ目の区切りを入れる
            let first_s = (i - w + 1) * w;

            // 2つ目の区切りは縦方向の中央に入れるか横方向の中央に入れるかの2通り
            // 横方向に区切りを入れる場合
            let second_s_h = w * ((h - i + w - 1) / 2);
            let third_s_h = h * w - first_s - second_s_h;
            // 縦方向に区切りを入れる場合
            let second_s_v = (h - i + w - 1) * (w / 2);
            let third_s_v = h * w - first_s - second_s_v;

            let s_diff_h =
                max(first_s, max(second_s_h, third_s_h)) - min(first_s, min(second_s_h, third_s_h));
            let s_diff_v =
                max(first_s, max(second_s_v, third_s_v)) - min(first_s, min(second_s_v, third_s_v));
            let s_diff = min(s_diff_h, s_diff_v);
            ans = if s_diff < ans { s_diff } else { ans };
        }
    }
    println!("{}", &ans);
}
