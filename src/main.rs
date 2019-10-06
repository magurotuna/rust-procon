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
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}

#[allow(unused_macros)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn a() {
    let s: String = read!(String);
    let k: usize = read!(usize);

    let c: Vec<char> = s.chars().collect();
    // ランレングス圧縮
    let mut runlen = Vec::new();
    let mut prev_c = c[0];
    let mut tmp = 0;
    for i in 0..c.len() {
        if prev_c == c[i] {
            tmp += 1;
            continue;
        } else {
            prev_c = c[i];
            runlen.push(tmp);
            tmp = 1;
        }
    }
    runlen.push(tmp);

    let mut ans = 0;
    // 先頭と末尾が同じ文字かどうかで場合分け
    if c.first().unwrap() != c.last().unwrap() {
        // 先頭と末尾が違う場合
        for r in runlen {
            if r == 1 {
                continue;
            } else {
                ans += r / 2;
            }
        }
        ans *= k;
    } else {
        // 先頭と末尾が同じ場合
        if runlen.len() > 1 {
            let r1 = &runlen[0];
            let r_middle = runlen.get(1..(runlen.len() - 1));
            let r2 = &runlen[runlen.len() - 1];

            match r_middle {
                Some(v) => {
                    for &r in v {
                        if r >= 2 {
                            ans += r / 2;
                        }
                    }
                    ans *= k;
                }
                None => (),
            }

            ans += *r1 / 2;
            ans += *r2 / 2;

            ans += (k - 1) * ((*r1 + *r2) / 2);
        } else {
            ans += (runlen[0] * k) / 2;
        }
    }
    println!("{}", ans);
}

fn b() {
    let n = read!(usize);
    let mut s = vec![];
    for i in 0..n {
        let t: Vec<i32> = read![[i32]];
        s.push(t);
    }
}

fn c() {
    unimplemented!();
}

fn d() {
    unimplemented!();
}

fn e() {
    unimplemented!();
}

fn f() {
    unimplemented!();
}

fn main() {
    b();
}
