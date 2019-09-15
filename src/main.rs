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
    let n = read!(usize);
    let a: Vec<usize> = read!(usize; n);

    let mut sort_a = a.clone();
    sort_a.sort();

    // 数字jがインデックスiにある場合 pos[j] = i とする
    let mut pos_orig = HashMap::new();
    let mut pos_sort = HashMap::new();

    for i in 0..n {
        pos_orig.insert(a[i], i);
        pos_sort.insert(sort_a[i], i);
    }

    let mut count = 0;
    for v in &a {
        // ソート後の位置とソート前の位置の差が偶数である場合は操作2の繰り返しにより移動可能
        // 奇数である場合は操作2を繰り返したあと最後に1回だけ操作1をする必要がある
        // そして「位置の差が奇数である」ような数字は必ず偶数個あり、この個数を2nとすると、操作1をおこなう回数はnとなる
        if pos_orig.get(v).unwrap() % 2 == pos_sort.get(v).unwrap() % 2 {
            continue;
        } else {
            count += 1;
        }
    }

    println!("{}", count / 2);
}
