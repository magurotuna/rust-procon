#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(deprecated)]

use std::cell::{Cell, Ref, RefCell, RefMut};
use std::cmp::{max, min, Ordering};
use std::collections::*;
use std::f64::consts::PI;
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
    unimplemented!();
}

fn b() {
    unimplemented!();
}

fn c() {
    let n = read!(usize);
    if n == 2 {
        println!("1");
        return;
    }
    if n == 3 || n == 4 {
        println!("2");
        return;
    }
    let mut max_div = 1;
    for i in 1..((n as f64).sqrt() as usize + 1) {
        //        println!("{}", i);
        if n % i == 0 {
            max_div = i;
        }
    }
    let t = n / max_div;

    //    debugln!("{} {}", max_div, t);

    println!("{}", max_div + t - 2);
}

fn d() {
    let (a, b, x) = read!(usize, usize, usize);
    let a = a as f64;
    let b = b as f64;
    let x = x as f64;
    let theta = ((b * b * a) / (2.0 * x)).atan();

    if b * ((PI / 2.0) - theta).tan() <= a {
        println!("{}", theta * 180.0 / PI);
        return;
    }

    let theta = ((2.0 * a * a * b - 2.0 * x) / a.powi(3)).atan();
    println!("{}", theta * 180.0 / PI);
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
