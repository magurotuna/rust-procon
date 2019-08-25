#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(deprecated)]

use std::cell::RefCell;
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

fn main() {
    let (n, m) = read!(usize, usize);
    let input: Vec<(usize, usize)> = read![usize, usize; m];
    let input: Vec<(usize, usize)> = input.iter().map(|&x| (x.0 - 1, x.1 - 1)).collect(); // 0オリジンに直す

    // i番目の頂点を頂点iと呼ぶことにする
    // 頂点iから頂点x, yに辺が伸びている場合は graph[i] = vec![x, y] であるとする
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..m {
        let (from, to) = input[i];
        graph[from].push(to);
    }

    // 通過していない頂点の番号を格納するベクタ
    let mut points: Vec<usize> = (0..n).rev().collect();

    let mut ans = 0;

    // 全頂点を通過するまでループ
    while !points.is_empty() {
        let p = points.pop().unwrap();
        let ne = &graph[p];
        fn dfs(cur: usize, neighbor: &Vec<usize>, ps: &Vec<usize>, g: &Vec<Vec<usize>>) -> bool {
            if ps.contains(&cur) {
                return false;
            }
            let mut ret = true;
            for nei in neighbor {
                if ps.contains(nei) {
                    ret = false;
                }
                // nei を通過済みとする
                let ps = ps
                    .iter()
                    .filter_map(|&x| if x != *nei { Some(x) } else { None })
                    .collect();

                // cur = nei, neighbor = graph[nei] として再帰
                ret = dfs(*nei, &g[*nei], &ps, g);
            }
            ret
        }
        ans = ans
            + if dfs(p, ne, &mut points, &graph) {
                1
            } else {
                0
            };
    }
    println!("{}", &ans);
}
