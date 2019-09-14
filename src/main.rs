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
            let root = self.find(px).unwrap(); // AtCoderでのRustバージョンではpxを分離しないとだめ
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
    let p: Vec<usize> = read![[usize]];
    let xy: Vec<(usize, usize)> = read!(usize, usize; m);

    let p = p.into_iter().map(|x| x - 1).collect::<Vec<_>>();
    // 数字jがp[i]に入っているとき pos[j] := i とする
    let mut pos = vec![0; n];
    for i in 0..n {
        pos[p[i]] = i;
    }

    let xy = xy
        .into_iter()
        .map(|x| {
            if x.0 > x.1 {
                (x.1 - 1, x.0 - 1)
            } else {
                (x.0 - 1, x.1 - 1)
            }
        })
        .collect::<Vec<(usize, usize)>>();

    let mut ds = DisjointSet::new(n);
    for (x, y) in xy {
        ds.unite(x, y);
    }

    let ans = (0..n).filter(|&x| ds.same(x, pos[x])).count();
    println!("{}", ans);
}
