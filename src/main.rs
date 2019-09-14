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

fn dfs(links: &Vec<Vec<usize>>, cur: usize, visited: &mut HashSet<usize>, nodes: usize) -> bool {
    visited.insert(cur);

    //    debug!(&visited, cur, nodes);

    if visited.len() == nodes {
        return true;
    }

    let nexts = &links[cur];
    let tmp_visited = visited.clone();
    for &n in nexts.iter().filter(|&x| !tmp_visited.contains(x)) {
        if dfs(links, n, visited, nodes) {
            return true;
        }
    }

    return false;
}

fn is_linked(links: &Vec<Vec<usize>>, nodes: usize) -> bool {
    let mut visited = HashSet::new();
    return dfs(links, 0, &mut visited, nodes);
}

fn main() {
    let (n, m) = read!(usize, usize);
    let edges: Vec<(usize, usize)> = read![usize,usize; m];
    let edges: Vec<(usize, usize)> = edges.into_iter().map(|(x, y)| (x - 1, y - 1)).collect();

    let mut link_count = 0;

    for i in 0..(edges.len()) {
        let mut links = vec![vec![]; n];

        for (from, to) in edges
            .iter()
            .enumerate()
            .filter(|&x| x.0 != i)
            .map(|(_, &x)| x)
        {
            links[from].push(to);
            links[to].push(from);
        }

        // この隣接状態(i本目を切断)でもグラフが連結かどうかを判定する
        if is_linked(&links, n) {
            link_count += 1;
        }
    }

    println!("{}", edges.len() - link_count);
}
