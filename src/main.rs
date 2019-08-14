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

fn no_land(p: &Vec<Vec<bool>>) -> bool {
    p.iter().all(|r| r.iter().all(|&c| !c))
}

fn dfs(p: &mut Vec<Vec<bool>>, a: usize, b: usize) {
    //    println!("{} {}", &a, &b);
    // 海に変える
    p[a][b] = false;
    // 上下左右を見て、陸地だったら再帰する
    let up_a = min(9, a + 1);
    let down_a = if a > 0 { a - 1 } else { 0 };
    let right_b = min(9, b + 1);
    let left_b = if b > 0 { b - 1 } else { 0 };

    //    println!("{} {} {} {}", up_a, down_a, right_b, left_b);

    // 上
    if p[up_a][b] {
        dfs(p, up_a, b);
    }
    // 下
    if p[down_a][b] {
        dfs(p, down_a, b);
    }
    // 左
    if p[a][left_b] {
        dfs(p, a, left_b);
    }
    // 右
    if p[a][right_b] {
        dfs(p, a, right_b);
    }
}

fn print(p: &Vec<Vec<bool>>) {
    for i in 0..10 {
        let v = &p[i];
        println!(
            "{:?}",
            v.iter()
                .map(|&b| if b { 'o' } else { 'x' })
                .collect::<Vec<char>>()
        );
    }
    println!("----------------------------");
}

fn main() {
    let input: Vec<String> = read![String; 10];
    let orig_p: Vec<Vec<bool>> = input
        .into_iter()
        .map(|r| {
            r.chars()
                .map(|c| if c == 'o' { true } else { false })
                .collect::<Vec<bool>>()
        })
        .collect();

    let mut ans = "NO";

    'first: for x in 0..10 {
        'second: for y in 0..10 {
            let mut p = orig_p.clone();
            // 注目してる座標が海だったら塗る
            if !p[x][y] {
                p[x][y] = true;
            }
            //            if x == 5 && y == 4 {
            //                print(&p);
            //            }

            // ある1つの陸地から深さ優先で探索していき、海に変えていく
            for a in 0..10 {
                for b in 0..10 {
                    if p[a][b] {
                        dfs(&mut p, a, b);
                        //                        if x == 5 && y == 4 {
                        //                            print(&p);
                        //                        }
                        // 全部が海になっているかを判定
                        // 海になっていたら 'YES' として全ループを抜ける
                        if no_land(&p) {
                            ans = "YES";
                            break 'first;
                        } else {
                            // この場合の(x, y)では島を連結することができなかったということなので、次の(x, y)を試す段階に進む
                            continue 'second;
                        }
                    }
                }
            }
        }
    }

    println!("{}", &ans);
}
