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

fn valid(c1: char, c2: char) -> bool {
    c1 == c2
}

fn main() {
    let s = read!(String);
    let t = read!(String);
    let sc = s.chars().collect::<Vec<_>>();
    let tc = t.chars().collect::<Vec<_>>();

    let mut map = HashMap::new();
    let mut imap = HashMap::new();

    for i in 0..sc.len() {
        let s = sc[i];
        let t = tc[i];
        match map.get(&s) {
            Some(&val) => {
                if !valid(val, t) {
                    println!("No");
                    return;
                }
            }
            None => {
                map.insert(s, t);
            }
        };

        match imap.get(&t) {
            Some(&val) => {
                if !valid(val, s) {
                    println!("No");
                    return;
                }
            }
            None => {
                imap.insert(t, s);
            }
        };
    }
    println!("Yes");
}
