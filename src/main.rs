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

fn a() {
    let s = read!(String);
    let t = read!(String);

    let c_s: Vec<char> = s.chars().collect::<Vec<char>>();
    let t_s: Vec<char> = t.chars().collect::<Vec<char>>();
    let mut ans = 0;
    for i in 0..c_s.len() {
        if c_s[i] == t_s[i] {
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn b() {
    let (a, b) = read!(usize, usize);
    if b == 1 {
        println!("0");
        return;
    }
    let mut ans = 1;
    let mut cnt = a;
    while cnt < b {
        ans += 1;
        cnt += a - 1;
    }
    println!("{}", ans);
}

fn c() {
    let n = read!(usize);
    let h: Vec<usize> = read![[usize]];

    // 最長部分減少列（ただし連続）の長さを求める その長さ-1が答え
    let mut max_len = 0;
    let mut len = 0;
    let mut prev_h = 0;
    for i in 0..n {
        if prev_h >= h[i] {
            prev_h = h[i];
            len += 1;
        } else {
            prev_h = h[i];
            if max_len < len {
                max_len = len;
            }
            len = 1;
        }
    }
    if max_len < len {
        max_len = len;
    }
    println!("{}", max_len - 1);
}

fn d() {
    let n = read!(usize);

    let ans = (n * (n - 1)) / 2;
    println!("{}", ans);
}

fn e() {
    let n = read!(usize);
    let mut a: Vec<VecDeque<usize>> = Vec::with_capacity(n);
    for i in 0..n {
        let v: Vec<usize> = read![[usize]];
        a.push(v.into_iter().map(|x| x - 1).collect());
    }

    let mut days = 0;
    // 対戦があるかを調べなければならないプレイヤーの番号。
    // 初日は当然全プレイヤーを調べなければならないが、2日目以降は「前日に対戦があって試合を消化したプレイヤー」のみを調べていく
    let mut player_for_search: Vec<usize> = (0..n).collect();

    while a.iter().any(|v| v.len() > 0) {
        days += 1;
        let mut combo = Vec::with_capacity(1000);
        'p1: for &i in player_for_search.iter().filter(|&&x| !a[x].is_empty()) {
            let opponent = a[i][0];
            let op_of_op = a[opponent][0];
            if i != op_of_op {
                continue 'p1;
            }
            let tup = if i <= opponent {
                (i, opponent)
            } else {
                (opponent, i)
            };
            if !combo.contains(&tup) {
                combo.push(tup);
            }
        }

        if combo.len() == 0 {
            println!("-1");
            return;
        }

        player_for_search.clear();

        for (p1, p2) in combo {
            a[p1].pop_front();
            a[p2].pop_front();
            player_for_search.push(p1);
            player_for_search.push(p2);
        }
    }
    println!("{}", days);
}

#[derive(PartialEq, PartialOrd)]
pub struct OrdFloat<T>(pub T);

impl<T: PartialEq> Eq for OrdFloat<T> {}

impl<T: PartialOrd> Ord for OrdFloat<T> {
    fn cmp(&self, other: &OrdFloat<T>) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

fn f() {
    let n = read!(usize);
    let mut engines: Vec<(i64, i64)> = Vec::with_capacity(n);
    for i in 0..n {
        engines.push(read!(i64, i64));
    }

    // engineのそれぞれのベクトルに対して、x軸の正方向となす角を計算
    let mut engines = engines
        .into_iter()
        .map(|(x, y)| {
            let cos = (x as f64) / ((x as f64).powi(2i32) + (y as f64).powi(2i32)).sqrt();
            let rad = if y >= 0 {
                cos.acos()
            } else {
                2.0 * std::f64::consts::PI - cos.acos()
            };
            (x, y, rad)
        })
        .collect::<Vec<(i64, i64, f64)>>();
    engines.sort_by_key(|&f| OrdFloat(f.2));

    let mut ans = 0.0;

    for i in 0..n {
        let vector = engines
            .iter()
            .filter(|&&x| {
                let &first_engine = &engines[i];
                first_engine.0 * x.0 + first_engine.1 * x.1 >= 0
            })
            .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

        let dist = ((vector.0.pow(2u32) + vector.1.pow(2u32)) as f64).sqrt();
        if dist > ans {
            ans = dist;
        }
    }

    println!("{}", ans);
}

fn main() {
    f();
}
