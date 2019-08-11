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

fn recurse(
    cur_str: String,
    target: &str,
    target_vec: &Vec<char>,
    d: &[&str; 2],
    e: &[&str; 2],
) -> bool {
    if target == &cur_str {
        return true;
    }

    let cur_len = cur_str.len();
    if cur_len > target_vec.len() {
        return false;
    }

    // target_vec の (cur_str.len() + 1) 文字目が d, e ではない場合は作れないのでfalseを返す
    match target_vec.get(cur_len) {
        Some(&c) => match c {
            'd' => {
                let mut s1 = cur_str.clone();
                let mut s2 = cur_str.clone();
                s1.push_str(d[0]);
                s2.push_str(d[1]);

                recurse(s1, target, target_vec, d, e) || recurse(s2, target, target_vec, d, e)
            }
            'e' => {
                let mut s1 = cur_str.clone();
                let mut s2 = cur_str.clone();
                s1.push_str(e[0]);
                s2.push_str(e[1]);

                recurse(s1, target, target_vec, d, e) || recurse(s2, target, target_vec, d, e)
            }
            _ => return false,
        },
        None => return false,
    }
}

fn resolve() {
    let s: String = read!(String);
    let mut t = String::new();
    let words_d = ["dream", "dreamer"];
    let words_e = ["erase", "eraser"];
    let s_chars = s.chars().vec();
    let s_len = s.len();

    let result = recurse(t, &s, &s_chars, &words_d, &words_e);
    println!("{}", if result { "YES" } else { "NO" });
}

fn main() {
    resolve();
}
