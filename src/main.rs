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

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),*) => {
        eprintln!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}

fn main() {
    let (n, m) = read!(usize, usize);

    // ある1つのマスAに着目すると、そのマスが接している8つのマスのうち何個にカードが存在しているかによって、マスAのカードがひっくり返される回数が確定する
    // 例えば接しているマス8つともカードがある場合は、マスAは9回ひっくり返されるので、最終的にウラになる
    // このようにして、マスAが接しているマス8のうちカードがあるマスの個数をNとすると
    // Nが奇数のときマスAの最終状態はオモテ
    // Nが偶数のときマスAの最終状態はウラ
    // となる。
    // したがって今回求める値は、「周囲を偶数個のカードマスで囲まれているマスの個数」

    // n <= m としても一般性を失わない
    let (n, m) = if n <= m { (n, m) } else { (m, n) };

    if n == 1 && m == 1 {
        println!("1");
        return;
    }
    if n == 1 {
        println!("{}", m - 2);
        return;
    }
    if n == 2 {
        println!("0");
        return;
    }
    // n >= 3 のとき
    // 両端の行および列は必ず奇数個のカードマスに囲まれているのでオモテになる
    // したがって内部のみ考えればいいが、内部は絶対に8つのカードマスに囲まれているので、ウラになる
    println!("{}", (n - 2) * (m - 2));
}
