#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
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

////////////////////////////////////////////////////////////////////////////////////////////////////
/// 飛ばし飛ばしで値を使うイテレータ Rust v1.28以降でしか使えないため独自実装
/// 簡易実装のためインデックスのオーバーフロー等は考慮していない
pub struct StepBy<I> {
    iter: I,
    step: usize,
    first_take: bool,
}
impl<I> StepBy<I> {
    pub fn new(iter: I, step: usize) -> StepBy<I> {
        StepBy {
            iter: iter,
            step: step - 1,
            first_take: true,
        }
    }
}
impl<I> Iterator for StepBy<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_take {
            self.first_take = false;
            self.iter.next()
        } else {
            self.iter.nth(self.step)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let inner_hint = self.iter.size_hint();
        if self.first_take {
            let f = |n| {
                if n == 0 {
                    0
                } else {
                    1 + (n - 1) / (self.step + 1)
                }
            };
            (f(inner_hint.0), inner_hint.1.map(f))
        } else {
            let f = |n| n / (self.step + 1);
            (f(inner_hint.0), inner_hint.1.map(f))
        }
    }
}
impl<I> ExactSizeIterator for StepBy<I> where I: ExactSizeIterator {}
////////////////////////////////////////////////////////////////////////////////////////////////////

trait IteratorExt: Iterator + Sized {
    // ref: https://qiita.com/vain0x/items/512784ff60ce599dccae#vec
    fn vec(self) -> Vec<Self::Item> {
        self.collect()
    }

    // step_by は v1.28.0 以降でしか使えないため独自実装
    fn stepby(self, step: usize) -> StepBy<Self>
    where
        Self: Sized,
    {
        StepBy::new(self, step)
    }
}

impl<T: Iterator> IteratorExt for T {}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// 逆順ソート ReverseがRust v1.19以降でしか使えないため独自実装
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Rev<T>(pub T);

impl<T: PartialOrd> PartialOrd for Rev<T> {
    fn partial_cmp(&self, other: &Rev<T>) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<T: Ord> Ord for Rev<T> {
    fn cmp(&self, other: &Rev<T>) -> Ordering {
        other.0.cmp(&self.0)
    }
}
// 逆順ソートここまで
////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
/// べき乗のmodを効率的に求める関数
/// ref: https://algorithmbeginner.blogspot.com/2018/02/blog-post_23.html
fn pow(x: u64, n: u64, modulo: u64) -> u64 {
    let mut res = 1;
    if n > 0 {
        res = pow(x, n / 2, modulo);
        if n % 2 == 0 {
            res = (res * res) % modulo;
        } else {
            res = (((res * res) % modulo) * x) % modulo;
        }
    }
    res
}
////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
/// ユークリッドの互除法
fn gcd(x: i64, y: i64) -> i64 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}
////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    let (n, m) = read!(usize, usize);
    let a_vec: Vec<usize> = read![[usize]];

    let table = vec![2usize, 5, 5, 4, 5, 6, 3, 7, 6];
    // 使用できる数字とそれを作るのに必要なマッチ本数のタプルのベクタ
    let mut use_digits = Vec::new();
    for a in a_vec {
        let num = table[a - 1];
        use_digits.push((a, num));
    }

    use_digits.sort_by_key(|&x| Rev(x.0));

    // dp[i] := i本のマッチを使って条件を満たす整数を作るときの最大桁数
    let mut dp = vec![MIN_INF; n + 1];

    // dp[0] = 0
    dp[0] = 0;

    for i in 2..(n + 1) {
        let mut tmp_max = MIN_INF;
        for &digit in &use_digits {
            let tmp = match i.checked_sub(digit.1) {
                Some(s) => dp[s] + 1,
                None => MIN_INF,
            };
            if tmp > tmp_max {
                tmp_max = tmp;
            }
        }
        dp[i] = tmp_max;
    }

    let mut s = String::new();

    let mut tmp_n = n;
    for i in 1..(dp[n] + 1) {
        for &d in &use_digits {
            let tmp = match tmp_n.checked_sub(d.1) {
                Some(sub) => sub,
                None => continue,
            };
            if dp[tmp] == dp[tmp_n] - 1 {
                s.push_str(&d.0.to_string());
                tmp_n = tmp;
                break;
            }
        }
    }

    println!("{}", &s);
}
