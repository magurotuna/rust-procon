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

fn main() {
    let s: String = read!(String);
    let s_len = s.len();

    let mut whole_sum = 0;

    for i in 0..1 << (s_len - 1) {
        let mut v = vec![false; s_len - 1];
        for j in 0..(s_len - 1) {
            // iのj番目ビットが立っているか
            if (1 << j) & i != 0 {
                v[j] = true;
            }
        }

        //        println!("{:?}", &v);

        let mut sum = 0;
        let mut used_count = 0; // すでにプラスによって分割され消費された文字の位置
        for k in 0..v.len() {
            if !v[k] {
                continue;
            }
            // + を入れる
            // used_count 〜 k までの数字charを1つの数字とみなして足す
            let d: usize = s[used_count..(k + 1)].parse().unwrap();
            sum += d;
            // used_count を更新
            used_count = k + 1;
        }
        // 最後にused_count から sの末尾 までの数字を足す
        let last: usize = s[used_count..s_len].parse().unwrap();
        sum += last;
        //        println!("{}", &sum);

        whole_sum += sum;
    }

    println!("{}", whole_sum);
}
