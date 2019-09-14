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

struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        DisjointSet {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> Option<usize> {
        if x > self.parent.len() {
            None
        } else if self.parent[x] == x {
            Some(x)
        } else {
            let px = self.parent[x];
            let root = self.find(px).unwrap();
            self.parent[x] = root;
            Some(root)
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let x_root = match self.find(x) {
            None => return,
            Some(val) => val,
        };
        let y_root = match self.find(y) {
            None => return,
            Some(val) => val,
        };

        if x_root == y_root {
            return;
        }

        if self.rank[x] < self.rank[y] {
            self.parent[x_root] = y_root;
        } else {
            self.parent[y_root] = x_root;
            if self.rank[x_root] == self.rank[y_root] {
                self.rank[x_root] += 1;
            }
        }
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        let x_root = match self.find(x) {
            None => return false,
            Some(val) => val,
        };
        let y_root = match self.find(y) {
            None => return false,
            Some(val) => val,
        };

        x_root == y_root
    }
}

fn main() {
    let (n, m) = read!(usize, usize);
    let edges: Vec<(usize, usize)> = read![usize,usize; m];
    let edges: Vec<(usize, usize)> = edges.into_iter().map(|(x, y)| (x - 1, y - 1)).collect();

    let mut ans = 0;

    'loo: for i in 0..m {
        let mut ds = DisjointSet::new(n);
        // i本目の辺がないとしたらどうなるか
        for (idx, &e) in edges.iter().enumerate() {
            if idx == i {
                continue;
            }
            ds.unite(e.0, e.1);
        }

        let root = ds.find(0);
        for j in 1..n {
            if ds.find(j) != root {
                // この場合はグラフが連結ではないということ よってi本目の辺は橋だった
                ans += 1;
                continue 'loo;
            }
        }
    }

    println!("{}", ans);
}
