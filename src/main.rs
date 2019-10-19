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
    let (a, b) = read!(i32, i32);
    println!("{}", max(0, a - 2 * b));
}

fn b() {
    let n = read!(usize);
    let d: Vec<usize> = read![[usize]];

    let mut ans = 0;
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            ans += d[i] * d[j];
        }
    }
    println!("{}", ans);
}

fn c() {
    let n = read!(usize);
    let s: String = read!(String);

    let c: Vec<char> = s.chars().collect();

    let mut prev = c[0];
    let mut ans = 0;
    for i in 0..n {
        if prev == c[i] {
            continue;
        } else {
            ans += 1;
            prev = c[i];
        }
    }
    println!("{}", ans + 1);
}

fn d() {
    let n = read!(usize);
    let mut l: Vec<usize> = read![[usize]];
    l.sort();
    l.reverse();

    let mut dp = vec![0; 3000]; // lから2つ選んで和がxになるような組合せの数

    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            let s = l[i] + l[j];
            dp[s] += 1;
        }
    }

    let mut under = vec![0; 3000];
    under[0] = 0;
    under[1] = 0;
    for i in 2..dp.len() {
        let t = under[i - 1];
        under[i] = t + dp[i];
    }

    let mut ans = n * (n - 1) * (n - 2) / 6;
    for i in 0..n {
        let max_v = l[i];
        ans -= under[max_v];
    }
    println!("{}", ans);
}

fn e() {
    let (n, m, l) = read!(usize, usize, i64);
    let mut abc = Vec::new();
    for i in 0..m {
        let t: Vec<usize> = read![[usize]];
        abc.push(t);
    }
    let q = read!(usize);
    let mut st = Vec::new();
    for i in 0..q {
        let t: Vec<usize> = read![[usize]];
        st.push(t);
    }

    let mut cost = vec![vec![1 << 60; n]; n];
    let mut next = vec![vec![1 << 60; n]; n];
    for i in 0..n {
        cost[i][i] = 0;
        for j in 0..n {
            next[i][j] = j;
        }
    }

    for row in abc {
        let from = row[0] - 1;
        let to = row[1] - 1;
        let cc = row[2];
        cost[from][to] = cc;
        cost[to][from] = cc;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let tmp = cost[i][k] + cost[k][j];
                if cost[i][j] > tmp {
                    cost[i][j] = tmp;
                    next[i][j] = next[i][k];
                }
            }
        }
    }

    'f: for query in st {
        let start = query[0] - 1;
        let end = query[1] - 1;
        if cost[start][end] == 1 << 60 {
            println!("-1");
            continue;
        }
        let mut ans = 0; // 補給回数
        let mut now = start;
        let mut tank: i64 = l;
        while now != end {
            let nexttown = next[now][end];
            // 次の街までの必要燃料がタンク最大容量を超えてたら無理
            debugln!("{}, {}", &l, &cost[now][nexttown]);
            if cost[now][nexttown] > l as usize {
                println!("-1");
                continue 'f;
            }

            // 燃料が足りない場合、満タンまで補充
            if cost[now][nexttown] > tank as usize {
                tank = l;
                ans += 1;
            }

            // 次の街まで移動
            tank -= cost[now][nexttown] as i64;
            now = nexttown;
        }
        println!("{}", ans);
    }
}

fn f() {
    unimplemented!();
}

fn main() {
    e();
}
