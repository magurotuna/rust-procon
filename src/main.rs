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
    let abc: Vec<(usize, usize, usize)> = read!(usize, usize, usize; n - 1);
    let (q, k) = read!(usize, usize);
    let k: usize = k - 1; // 0-based
    let xy: Vec<(usize, usize)> = read!(usize, usize; q);

    let abc = abc
        .into_iter()
        .map(|x| (x.0 - 1, x.1 - 1, x.2))
        .collect::<Vec<(usize, usize, usize)>>();
    let xy = xy
        .into_iter()
        .map(|x| (x.0 - 1, x.1 - 1))
        .collect::<Vec<_>>();

    // 経由点であるkから各頂点への距離を求める
    // dist[i] := kからiまでの距離
    let mut dist = vec![INF as usize; n];
    dist[k] = 0;

    let mut links = vec![vec![]; n];
    for &r in &abc {
        let from = r.0;
        let to = r.1;
        let cost = r.2;
        links[from].push((to, cost));
        links[to].push((from, cost));
    }

    // bfs
    let mut deque = VecDeque::new();
    for &l in &links[k] {
        deque.push_back((k, l.0, l.1));
    }
    while let Some(cur) = deque.pop_front() {
        if dist[cur.1] != INF as usize {
            // もう最短距離計算済み
            continue;
        }
        // next
        for &l in &links[cur.1] {
            deque.push_back((cur.1, l.0, l.1));
        }

        dist[cur.1] = dist[cur.0] + cur.2;
    }

    // query processing
    for &query in &xy {
        println!("{}", dist[query.0] + dist[query.1]);
    }
}
