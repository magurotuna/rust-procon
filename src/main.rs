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
    let s = read!(String);
    let c: Vec<char> = s.chars().collect();
    let mut prev_c = '*';
    let mut runlen: Vec<(char, usize)> = vec![];
    for cc in c {
        if prev_c == cc {
            runlen.last_mut().unwrap().1 += 1;
        } else {
            runlen.push((cc, 1));
            prev_c = cc;
        }
    }
    let mut ans = 0;
    let mut index = 0;

    if runlen[0].0 == '>' {
        let r = runlen[0];
        ans += r.1 * (r.1 + 1) / 2;
        index = 1;
    }

    while index < runlen.len() {
        let next = index + 1;
        let r = runlen[index];

        if next == runlen.len() {
            ans += r.1 * (r.1 + 1) / 2;
        } else {
            let r2 = runlen[next];
            // rが増加列、r2が減少列
            if r.1 < r2.1 {
                ans += r.1 * (r.1 - 1) / 2;
                ans += r2.1 * (r2.1 + 1) / 2;
            } else if r.1 >= r2.1 {
                ans += r.1 * (r.1 + 1) / 2;
                ans += r2.1 * (r2.1 - 1) / 2;
            }
        }
        index += 2;
    }

    println!("{}", ans);
}

fn b() {
    let n = read!(usize);
    let mut lr = vec![];
    for i in 0..n {
        let t = read!(usize, usize);
        lr.push((t.0 - 1, t.1 - 1));
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
    a();
}
