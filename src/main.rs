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

fn dfs(cur_pos: usize, visited: Vec<usize>, link: &Vec<Vec<usize>>) -> i32 {
    let mut count = 0;
    let mut new_visited = visited.clone();
    new_visited.push(cur_pos);

    if new_visited.len() == link.len() {
        return 1;
    }

    for &next in link[cur_pos].iter().filter(|&x| !new_visited.contains(x)) {
        count += dfs(next, new_visited.clone(), link);
    }

    count
}

fn main() {
    let (n, m) = read!(usize, usize);
    let ab: Vec<(usize, usize)> = read!(usize, usize; m);

    let mut nodes = vec![vec![]; n];
    for i in 0..m {
        let from = ab[i].0;
        let to = ab[i].1;
        nodes[from - 1].push(to - 1);
        nodes[to - 1].push(from - 1);
    }

    println!("{}", dfs(0, vec![], &nodes));
}
