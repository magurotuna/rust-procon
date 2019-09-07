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

fn a() {
    let n = read!(usize);
    println!("{}", n.pow(3));
}

fn b() {
    let n = read!(usize);
    let a = read!([usize]);
    let b = read!([usize]);
    let c = read!([usize]);
    let mut ans = 0;
    for i in 0..n {
        if i == 0 {
            ans += b[a[i] - 1];
        } else {
            ans += b[a[i] - 1];
            if a[i - 1] + 1 == a[i] {
                ans += c[a[i - 1] - 1];
            }
        }
    }
    println!("{}", ans);
}

fn c() {
    let n = read!(usize);
    let b: Vec<usize> = read![[usize]];

    //    if n == 2 {
    //        println!("{}", b[0] * 2);
    //        return;
    //    }

    let mut v = Vec::with_capacity(n + 1);
    v.push(INF as usize);
    v.extend(b);
    v.push(INF as usize);

    let mut ans = 0;
    for i in 1..(n + 1) {
        ans += min(v[i - 1], v[i]);
    }
    println!("{}", ans);

    //    let mut a = Vec::with_capacity(n);
    //    if b[0] <= b[1] {
    //        a.push(b[0]);
    //        a.push(b[0]);
    //    } else {
    //        a.push(b[1]);
    //        a.push(b[0]);
    //    }
    //    for i in 0..(n - 1) {
    //
    //    }
}

fn d() {
    let (n, k) = read!(usize, usize);
    let s: String = read!(String);

    let c = s.chars().collect::<Vec<char>>();
    // 全員が同じ方向を向いていない限り、操作によって絶対に全体の幸福度を少なくとも1は上げることができる
    // 現時点で幸福な人は、操作区間の両端以外に含まれている場合、操作後も幸福を継続できる
    // 操作によって変化する可能性のある幸福度は、操作区間の両端の人の幸福度のみ。
    // したがって1回の操作で変化しうる幸福度は高々2であり、最初の考察と合わせて、1回の操作で変化する幸福度は 1 or 2 である（ただしすべての人が同じ方向を向いていない場合）
    // 1回の操作で幸福度を2あげるような区間の選び方ができるかを検討する

    // LとRの「変わり目」の数をカウントする
    let mut count = 0;
    for i in 1..c.len() {
        if c[i] != c[i - 1] {
            count += 1;
        }
    }
    // 操作1回で変わり目は最大で2個減る。
    if 2 * k >= count {
        println!("{}", n - 1);
        return;
    }
    // n - 変わり目の個数 - 1 が幸福度
    let d = count - 2 * k;
    println!("{}", n - d - 1);
}

fn e() {
    unimplemented!();
}

fn f() {
    unimplemented!();
}

fn main() {
    d();
}
