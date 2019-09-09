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

    // (問題数, コンプリートボーナス, コンプリートボーナスなしの場合の1問あたりの点数, ありの場合の1問あたりの点数)
    let mut pc: Vec<_> = pc
        .into_iter()
        .enumerate()
        .map(|(i, (x, y))| {
            let p_per_q = (i as i32 + 1) * 100;
            let comp_point = p_per_q * x + y;
            let point_per_q = comp_point as f64 / x as f64;
            (x, y, p_per_q, point_per_q)
        })
        .collect();

    let mut pc2 = pc.clone();

    // ボーナスありでの1問あたりの点数によるソート
    pc.sort_by_key(|x| OrdFloat(x.3));
    // ボーナスなしでの1問あたりの点数によるソート（自明だけど..）
    pc2.sort_by_key(|x| x.2);

    debug!(pc);
    debug!(pc2);

    let mut rest: i32 = g;
    let mut ans = 0;

    while rest > 0 {
        let &last = pc.last().unwrap();
        let all_point = last.2 * last.0 + last.1;
        if rest - all_point >= 0 {
            rest -= all_point;
            ans += last.0;
            pc.pop();
            pc2 = pc2.into_iter().filter(|x| x.2 != last.2).collect();
            continue;
        } else {
            let last2 = pc2.pop().unwrap();

            let num = if rest % last2.2 == 0 {
                rest / last2.2
            } else {
                (rest / last2.2) + 1
            };

            if num <= last2.0 {
                ans += min(num, last.0);
                break;
            } else {
                rest -= last2.2 * last2.0 + last2.1;
                ans += num;
                pc = pc.into_iter().filter(|x| x.2 != last2.2).collect();
                continue;
            }
        }
    }

    println!("{}", ans);
}
