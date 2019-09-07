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
    let (n, m) = read!(usize, usize);
    let mut list: Vec<(usize, usize, usize)> = Vec::with_capacity(m);
    for i in 0..m {
        let input = read!(usize, usize);
        list.push((i, input.0, input.1));
    }

    list.sort_by(|x, y| {
        if x.1 != y.1 {
            x.1.cmp(&y.1)
        } else {
            x.2.cmp(&y.2)
        }
    });

    let mut numbered = Vec::with_capacity(m);
    let mut pref_number = list.first().unwrap().1;
    let mut city_birth_order = 0;
    for &city in &list {
        if pref_number == city.1 {
            city_birth_order += 1;
        } else {
            pref_number = city.1;
            city_birth_order = 1;
        }
        numbered.push((
            city.0,
            format!("{:>06}{:>06}", &pref_number, &city_birth_order),
        ));
    }

    numbered.sort_by_key(|x| x.0);

    for ans in &numbered {
        println!("{}", ans.1);
    }
}
