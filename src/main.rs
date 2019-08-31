#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(deprecated)]

use std::cell::{Cell, Ref, RefCell};
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

#[derive(Debug)]
struct Node {
    number: usize,
    neighbors: RefCell<Vec<usize>>,
    dist_from_start: Cell<usize>,
    visited: Cell<bool>,
}

impl Node {
    fn new(n: usize) -> Node {
        Node {
            number: n,
            neighbors: RefCell::new(vec![]),
            dist_from_start: Cell::new(INF as usize),
            visited: Cell::new(false),
        }
    }

    fn update_neighbor(&mut self, neighbors: &[usize]) {
        let mut x = self.neighbors.borrow_mut();
        for &n in neighbors {
            x.push(n);
        }
    }

    fn add_neighbor(&mut self, neighbor: usize) {
        let mut x = self.neighbors.borrow_mut();
        x.push(neighbor);
    }

    fn update_dist(&self, value: usize) {
        self.dist_from_start.set(value);
    }

    fn is_visited(&self) -> bool {
        self.visited.get()
    }

    fn set_visited(&self) {
        self.visited.set(true);
    }
}

fn bfs(towns: &Vec<Node>, current: usize, goal: usize) -> usize {
    if current == goal {
        return 1;
    }
    let mut queue = VecDeque::new();
    let town = towns.get(current).unwrap();
    for &n in town.neighbors.borrow().iter() {
        queue.push_front(n);
    }
    let mut count = 0;
    while let Some(next) = queue.pop_back() {
        count += bfs(towns, next, goal);
    }
    count
}

fn main() {
    let n = read!(usize);
    let (a, b) = read!(usize, usize);
    let m = read!(usize);
    let roads: Vec<(usize, usize)> = read![usize, usize; m];
    // 道路の接続を示す町の番号を0-basedに直す
    let roads: Vec<(usize, usize)> = roads.into_iter().map(|(x, y)| (x - 1, y - 1)).collect();
    let (a, b): (usize, usize) = (a - 1, b - 1);

    let mut towns = Vec::with_capacity(n);
    for i in 0..n {
        let node = if i == a {
            let node = Node::new(i);
            node.update_dist(0);
            node
        } else {
            Node::new(i)
        };
        towns.push(node);
    }
    for i in 0..m {
        let (from, to) = roads[i];
        towns[from].add_neighbor(to);
        towns[to].add_neighbor(from);
    }

    let mut queue = VecDeque::new();
    queue.push_front(a);
    while let Some(next) = queue.pop_back() {
        let next_town = towns.get(next).unwrap();
        // ゴールに着いた場合
        if next == b {}
        // 訪問済みだったらスキップ
        if next_town.is_visited() {
            continue;
        }
        // 訪問済みにする
        next_town.set_visited();
        // 探索リストに追加
        for &n in next_town.neighbors.borrow().iter() {
            queue.push_front(n);
        }
    }

    println!("{}", bfs(&towns, a, b));
}
