#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(deprecated)]

use std::cell::{Cell, Ref, RefCell};
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

#[derive(Debug)]
struct Person {
    number: i64,
    grade: i64,
    salary: Cell<i64>,
    children: RefCell<Vec<i64>>,
}

fn dfs(co: &Vec<Person>, children: Ref<Vec<i64>>) -> i64 {
    if children.len() == 0 {
        return 1;
    }
    let children_salaries = children
        .iter()
        .map(|&x| dfs(co, co[x as usize - 1].children.borrow()))
        .collect::<Vec<i64>>();
    let &max_sal = children_salaries.iter().max().unwrap();
    let &min_sal = children_salaries.iter().min().unwrap();
    max_sal + min_sal + 1
}

fn main() {
    let n = read!(usize);
    let b: Vec<usize> = read![usize; (n - 1)];

    let mut com = vec![Person {
        number: 1,
        grade: 1,
        salary: Cell::new(0),
        children: RefCell::new(vec![]),
    }];

    for (i, &e) in b.iter().enumerate() {
        let parent_grade;
        {
            let parent = com.get(e - 1).unwrap();
            let mut p_children = parent.children.borrow_mut();
            p_children.push((i + 2) as i64);
            parent_grade = parent.grade;
        }
        com.push(Person {
            number: i as i64,
            grade: parent_grade + 1,
            salary: Cell::new(0),
            children: RefCell::new(vec![]),
        });
    }

    // 深さ優先探索
    let takahashi_sal = dfs(&com, com[0].children.borrow());
    println!("{}", &takahashi_sal);
}
