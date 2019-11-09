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
    let n = read!(usize);
    if n <= 2 {
        println!("0");
        return;
    }
    println!("{}", ((n - 1) as f64 / 2 as f64).floor() as u64);
}

/// 累乗のmod
pub fn mod_pow(x: u64, n: u64, m: u64) -> u64 {
    let mut res = 1;
    let mut x = x % m;
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            res = (res * x) % m;
        }
        x = (x * x) % m;
        n >>= 1;
    }
    res
}

fn b() {
    let n = read!(usize);
    let d: Vec<usize> = read![[usize]];

    if d[0] > 0 {
        println!("0");
        return;
    }

    if n == 1 {
        println!("1");
        return;
    }

    let mut count = vec![0; n];
    for i in 0..d.len() {
        if i > 0 && d[i] == 0 {
            println!("0");
            return;
        }
        count[d[i]] += 1;
    }

    let mut ans = 1u64;
    let mo = 998244353u64;
    for i in 1..(n - 1) {
        if i == 1 {
            // ans += count[i] % mo;
            continue;
        } else {
            let tmp = mod_pow(count[i - 1], count[i], mo);
            ans = ans % mo;
            ans *= tmp;
        }
    }

    println!("{}", ans);
}

fn c() {
    let n = read!(usize);
    let a: Vec<usize> = read![[usize]];
    let b: Vec<usize> = read![[usize]];
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn d() {
    let (n, m) = read!(usize, usize);
    let mut lrc: Vec<(usize, usize, usize)> = Vec::new();
    for _ in 0..m {
        let t = read!(usize, usize, usize);
        lrc.push((t.0 - 1, t.1 - 1, t.2));
    }

    // コストの昇順でソート
    lrc.sort_by_key(|&x| x.2);
    debugln!("{:?}", &lrc);

    let mut dist = vec![(INF as usize, INF as usize, 0usize); n - 1];

    for i in 0..m {
        let from = lrc[i].0;
        let to = lrc[i].1;
        let cost = lrc[i].2;
        for j in from..to {
            // if i > 0 && dist[j] != INF as usize {
            //     continue;
            // }
            dist[j].0 = if dist[j].0 > cost { cost } else { dist[j].0 };
            dist[j].1 = if dist[j].1 > from { from } else { dist[j].1 };
            dist[j].2 = if dist[j].2 < to { to } else { dist[j].2 };
        }
    }

    debugln!("{:?}", &dist);

    let mut cost2 = vec![INF as usize; n];
    cost2[0] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        position: 0,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == n {
            break;
        }
        debugln!("cost: {}, position: {}", cost, position);

        for i in (dist[position].1)..(dist[position].2 + 1) {
            // i: 行き先 position: 今いるところ
            if i == position {
                continue;
            }

            // position -> i への距離
            let actual_dist = max(dist[position].0, dist[i - 1].0);

            if cost2[i] > actual_dist + cost {
                cost2[i] = actual_dist + cost;
                debugln!("{} renewed: {}", i, cost2[i]);
                heap.push(State {
                    cost: cost + actual_dist,
                    position: i,
                });
            }
        }
    }

    debugln!("{:?}", &cost2);
}

fn main() {
    d();
}
