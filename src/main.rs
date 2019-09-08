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
    let (a, b, c, d, e, f) = read!(i32, i32, i32, i32, i32, i32);

    // 制約を考慮にいれると、水の入れ方は100gから2900gまで100g刻みでの29通りしかありえない（実現可能性は置いておいて）
    // この29通りの入れ方が実現可能であるかどうかをまず求める
    let mut available_water = Vec::with_capacity(30);
    'out: for i in 1..30 {
        let water = 100 * i;
        if water >= f {
            break;
        }

        for j in 0..((i / a) + 1) {
            for k in 0..((i / b) + 1) {
                if 100 * a * j + 100 * b * k == water {
                    available_water.push(water);
                    continue 'out;
                }
            }
        }
    }

    let mut max_dens = 0.0;
    let mut ans1 = 100 * a;
    let mut ans2 = 0;

    // 実現可能な水の質量に対して砂糖をどれだけ入れるかを考える
    for &w in &available_water {
        let max_sugar = w * e / 100;

        let mut actual_max_sugar = 0;

        's1: for i in 0..((max_sugar / c) + 1) {
            for j in 0..((max_sugar / d) + 1) {
                let sugar_weight = c * i + d * j;
                // ビーカーの質量制限
                if w + sugar_weight > f {
                    continue;
                }

                // 最大溶解量を超える場合
                if sugar_weight > max_sugar {
                    continue;
                }

                // 最大溶解量まで砂糖を溶かすことができる場合
                if sugar_weight == max_sugar {
                    actual_max_sugar = sugar_weight;
                    break 's1;
                }

                // 最大までは解かせない場合
                if sugar_weight > actual_max_sugar {
                    actual_max_sugar = sugar_weight;
                }
            }
        }

        let dens = (100.0 * actual_max_sugar as f64) / ((w + actual_max_sugar) as f64);
        if dens > max_dens {
            max_dens = dens;
            ans1 = w + actual_max_sugar;
            ans2 = actual_max_sugar;
        }
    }

    println!("{} {}", ans1, ans2);
}
