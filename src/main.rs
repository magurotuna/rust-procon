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
    let N: usize = read!(usize);

    // N 以下、1以上の奇数の個数をMとすると 答えは M / N

    if N == 1 {
        println!("{}", 1.0);
        return;
    }

    if N % 2 == 0 {
        println!("{}", 0.50);
    } else {
        println!("{}", ((N + 1) as f64) / ((2 * N) as f64));
    }
}

fn b() {
    let (n, k) = read!(usize, usize);
    let mut h: Vec<usize> = read![[usize]];
    h.sort();
    h.reverse();

    let mut ans = 0;
    for hh in h {
        if hh >= k {
            ans += 1;
        } else {
            println!("{}", ans);
            return;
        }
    }
    println!("{}", ans);
}

fn c() {
    let n = read!(usize);
    let a: Vec<usize> = read![[usize]];

    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        v.push((i + 1, a[i]));
    }
    v.sort_by_key(|&x| x.1);

    let vv = v
        .into_iter()
        .map(|x| x.0.to_string())
        .collect::<Vec<String>>();
    println!("{}", vv.join(" "));
}

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub trait Prime {
    fn lower_primes(&self) -> Vec<usize>;
    fn factorize(&self) -> HashMap<usize, usize>;
}

impl Prime for usize {
    /// エラトステネスの篩を用いてself以下の素数を求める
    fn lower_primes(&self) -> Vec<usize> {
        let &this = self;
        let mut v = Vec::new();
        if this < 2 {
            return v;
        }
        let mut deque = (2..(this + 1)).collect::<VecDeque<usize>>();

        let mut p = match deque.pop_front() {
            Some(x) => x,
            None => return v,
        };
        v.push(p);
        while p as f64 <= (this as f64).sqrt() {
            deque = deque.iter().filter(|&&x| x % p != 0).map(|x| *x).collect();
            p = match deque.pop_front() {
                Some(x) => x,
                None => return v,
            };
            v.push(p);
        }
        for n in deque {
            v.push(n);
        }
        v
    }

    /// エラトステネスの篩を用いてselfを素因数分解する
    fn factorize(&self) -> HashMap<usize, usize> {
        let mut ret = HashMap::new();
        let primes = ((*self as f64).sqrt() as usize).lower_primes();

        let mut tmp = *self;
        for prime in primes {
            while tmp % prime == 0 {
                tmp /= prime;
                *ret.entry(prime).or_insert(0) += 1;
            }
        }
        if tmp > 1 {
            *ret.entry(tmp).or_insert(0) += 1;
        }
        ret
    }
}

fn d() {
    let (a, b) = read!(u64, u64);

    let gcdab = gcd(a, b);

    let devisors = (gcdab as usize).factorize();
    println!("{}", devisors.len() + 1);
}

fn e() {
    let (n, m) = read!(usize, usize);
    let mut ab: Vec<(usize, usize)> = vec![];
    let mut c: Vec<Vec<usize>> = vec![];
    for i in 0..m {
        let t1 = read!(usize, usize);
        let t2 = read![[usize]];
        ab.push(t1);
        c.push(t2);
    }

    // dp[i][j] := i番目までのカギの中からいくつかのカギを選んで宝箱1, 2, ..., jを開ける場合の、選んだカギの価格の合計の最小値
    //
}

fn f() {
    unimplemented!();
}

fn main() {
    a();
}
