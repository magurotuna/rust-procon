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

fn main() {
    let s = read!(String);
    let t = read!(String);
    let sc = s.chars().collect::<Vec<_>>();
    let tc = t.chars().collect::<Vec<_>>();

    let mut alpha = HashMap::new();
    for i in 0..sc.len() {
        (*alpha.entry(sc[i]).or_insert(vec![])).push(i);
    }

    let mut set: HashSet<usize> = HashSet::new();
    for i in 0..sc.len() {
        if sc[i] == tc[i] {
            continue;
        }

        if set.contains(&i) {
            // すでに1回、操作を適用済みである場合は、もう不可能
            continue;
        }

        // sc[i] -> tc[i] に置換してチェック
        for &scidx in alpha.get(&sc[i]).unwrap() {
            // 検証済みインデックスを登録
            set.insert(scidx);
            if tc[scidx] != tc[i] {
                println!("No");
                return;
            }
        }

        // sc中の tc[i] を sc[i] に置換してチェック
        match alpha.get(&tc[i]) {
            None => (),
            Some(arr) => {
                for &tcidx in arr {
                    set.insert(tcidx);
                    if tc[tcidx] != sc[i] {
                        println!("No");
                        return;
                    }
                }
            }
        };
    }

    println!("Yes");
}
