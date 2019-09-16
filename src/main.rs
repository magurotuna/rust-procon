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
    let s: String = read!(String);
    let t = read!(usize);

    let sc: Vec<char> = s.chars().collect();

    let mut hm = HashMap::new();
    hm.insert('L', 0i32);
    hm.insert('R', 0i32);
    hm.insert('U', 0i32);
    hm.insert('D', 0i32);
    hm.insert('?', 0i32);
    for c in sc {
        *hm.entry(c).or_insert(0) += 1;
    }

    let pos = (
        *hm.get(&'R').unwrap() - *hm.get(&'L').unwrap(),
        *hm.get(&'U').unwrap() - *hm.get(&'D').unwrap(),
    );

    let ques = *hm.get(&'?').unwrap();

    if t == 1 {
        // ?をうまく調整することで絶対に距離を伸ばすことができるから、?の数だけ加算する
        println!("{}", pos.0.abs() + pos.1.abs() + ques);
    } else {
        // 原点より近い点はないことに注意が必要
        let tmp = pos.0.abs() + pos.1.abs() - ques;
        if tmp < 0 {
            // まず?を使って原点に戻る 戻ったあとquesが何個残っているか
            let rest_ques = ques - pos.0.abs() - pos.1.abs();
            // 残っているquesが偶数ならウロウロすることで原点に戻ってこれるが、奇数ならがんばっても最後には距離1のところにいってしまう
            println!("{}", if rest_ques % 2 == 0 { 0 } else { 1 })
        } else {
            println!("{}", tmp);
        }
    }
}
