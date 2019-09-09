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

#[derive(PartialEq, PartialOrd)]
pub struct OrdFloat<T>(pub T);

impl<T: PartialEq> Eq for OrdFloat<T> {}

impl<T: PartialOrd> Ord for OrdFloat<T> {
    fn cmp(&self, other: &OrdFloat<T>) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

fn main() {
    let (d, g) = read!(i32, i32);
    let pc: Vec<(i32, i32)> = read![i32, i32; d];

    let mut ans = 100000000;
    // 完全に解く問題をbit全探索する
    for i in 0..2i32.pow(d as u32) {
        let mut rest = g;
        let mut count = 0;
        let mut deleted_idx = vec![];
        for j in 0..d {
            if 1 << j & i != 0 {
                // 100(j+1) 点の問題を全部解く
                rest -= 100 * (j + 1) * pc[j as usize].0 + pc[j as usize].1;
                count += pc[j as usize].0;
                deleted_idx.push(j);
            }
        }

        if rest <= 0 {
            ans = min(ans, count);
            continue;
        }

        let exist: Vec<_> = pc
            .iter()
            .enumerate()
            .filter(|&(x, _)| !deleted_idx.contains(&(x as i32)))
            .map(|(x, &y)| (x, y.0, y.1))
            .collect();

        for &e in &exist {
            let point_per_q = 100 * (e.0 as i32 + 1);
            let num = if rest % point_per_q == 0 {
                rest / point_per_q
            } else {
                (rest / point_per_q) + 1
            };
            if num < e.1 {
                ans = min(ans, count + num);
            } else {
                continue;
            }
        }
    }

    println!("{}", ans);
}
