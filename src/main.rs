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
    let s = read!(String);

    let n: i32 = match s.parse() {
        Ok(k) => k,
        Err(_) => {
            println!("error");
            999999999
        }
    };
    if n == 999999999 {
        return;
    }
    println!("{}", n * 2);
}

fn b() {
    let n = read!(usize);
    let a = read!(usize; n);

    for i in 1..n {
        if a[i - 1] == a[i] {
            println!("stay");
        } else if a[i] < a[i - 1] {
            println!("down {}", a[i - 1] - a[i]);
        } else {
            println!("up {}", a[i] - a[i - 1]);
        }
    }
}

fn c() {
    let mut input = read![[usize]];

    input.sort();

    println!("{}", input[3]);
}

fn d() {
    let n = read!(usize);
    let a = read!(usize; n);

    let mut v = vec![0; n + 1];
    for aa in a {
        v[aa] += 1;
    }
    let mut correct = true;
    let mut from = 999999999;
    let mut to = 999999999;
    for (idx, val) in v.into_iter().enumerate() {
        if idx == 0 {
            continue;
        }
        if val == 2 {
            to = idx;
            correct = false;
            continue;
        }
        if val == 0 {
            from = idx;
            correct = false;
            continue;
        }
    }

    if correct {
        println!("Correct");
    } else {
        println!("{} {}", to, from);
    }
}

fn e() {
    let (n, q) = read!(usize, usize);
    let mut s = Vec::with_capacity(q);
    for i in 0..q {
        let tmp = read!(String);
        let tmp = tmp.split_whitespace().collect::<Vec<_>>();
        let op_type = match tmp[0] {
            "1" => OperationType::Follow,
            "2" => OperationType::Followall,
            "3" => OperationType::Followfollow,
            _ => panic!("hoge"),
        };
        s.push(Operation {
            op_type: op_type,
            from: tmp[1].parse::<usize>().unwrap() - 1,
            to: match op_type {
                OperationType::Follow => Some(tmp[2].parse::<usize>().unwrap() - 1),
                _ => None,
            },
        });
    }

    let mut f = vec![vec!['N'; n]; n];

    for ss in s {
        ss.exec(&mut f, n);
    }
    for ii in 0..f.len() {
        let ff = f[ii].clone();
        println!(
            "{}",
            ff.into_iter()
                .enumerate()
                .map(|(i, x)| if ii == i {
                    "N".to_string()
                } else {
                    x.to_string()
                })
                .collect::<Vec<_>>()
                .join("")
        )
    }
}

#[derive(Clone, Copy, Debug)]
enum OperationType {
    Follow,
    Followall,
    Followfollow,
}

#[derive(Debug)]
struct Operation {
    op_type: OperationType,
    from: usize,
    to: Option<usize>,
}

impl Operation {
    fn exec(&self, f: &mut Vec<Vec<char>>, n_users: usize) {
        match self.op_type {
            OperationType::Follow => {
                let from = self.from;
                let to = self.to.unwrap();
                if from != to {
                    f[from][to] = 'Y';
                }
            }
            OperationType::Followall => {
                for i in 0..n_users {
                    if i == self.from {
                        continue;
                    }
                    if f[i][self.from] == 'Y' {
                        f[self.from][i] = 'Y';
                    }
                }
            }
            OperationType::Followfollow => {
                let cloned = f[self.from].clone(); // users who are followed by self.from
                for (following, _) in cloned.into_iter().enumerate().filter(|&(_, x)| x == 'Y') {
                    let cloned2 = f[following].clone();
                    for (followfollow, _) in cloned2.iter().enumerate().filter(|&(_, &x)| x == 'Y')
                    {
                        if followfollow == self.from {
                            continue;
                        }
                        f[self.from][followfollow] = 'Y';
                    }
                }
            }
        }
    }
}

fn f() {
    let s = read!(String);

    let c: Vec<char> = s.chars().collect();

    let mut lower_words = Vec::with_capacity(50000);

    let mut tmp_word = "".to_string();
    for cc in c {
        if cc.is_uppercase() {
            if tmp_word.len() == 0 {
                tmp_word.push_str(&cc.to_lowercase().collect::<String>());
                continue;
            } else {
                tmp_word.push_str(&cc.to_lowercase().collect::<String>());
                lower_words.push(tmp_word.clone());
                tmp_word = "".to_string();
            }
        } else {
            tmp_word.push(cc);
        }
    }
    lower_words.sort();
    println!(
        "{}",
        lower_words
            .into_iter()
            .map(|word| {
                let mut ch: Vec<_> = word.chars().collect();
                let l = ch.len();
                let c_first = ch[0];
                let c_last = ch[ch.len() - 1];
                ch[0] = c_first
                    .to_uppercase()
                    .collect::<String>()
                    .chars()
                    .collect::<Vec<_>>()[0];
                ch[l - 1] = c_last
                    .to_uppercase()
                    .collect::<String>()
                    .chars()
                    .collect::<Vec<_>>()[0];
                ch.into_iter().collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("")
    );
}

fn g() {
    let n = read!(usize);
    let mut a = Vec::with_capacity(n - 1);
    for i in 0..(n - 1) {
        let mut base = vec![0; i];
        let mut tmp = read![[i64]];
        base.append(&mut tmp);
        a.push(base);
    }

    let ans = dfs_g(n, 0, vec![], &a);
    println!("{}", ans);
}

fn dfs_g(goal: usize, cur_depth: usize, group: Vec<usize>, point_list: &Vec<Vec<i64>>) -> i64 {
    if cur_depth == goal {
        return calc_point(&group, point_list);
    }

    let result1 = {
        let mut copied = group.clone();
        copied.push(1);
        dfs_g(goal, cur_depth + 1, copied, point_list)
    };
    let result2 = {
        let mut copied = group.clone();
        copied.push(2);
        dfs_g(goal, cur_depth + 1, copied, point_list)
    };
    let result3 = {
        let mut copied = group.clone();
        copied.push(3);
        dfs_g(goal, cur_depth + 1, copied, point_list)
    };
    max(result1, max(result2, result3))
}

fn calc_point(group: &Vec<usize>, point_list: &Vec<Vec<i64>>) -> i64 {
    let mut group1_employees = group
        .iter()
        .enumerate()
        .filter_map(|(idx, &g)| if g == 1 { Some(idx) } else { None })
        .collect::<Vec<_>>();
    let mut group2_employees = group
        .iter()
        .enumerate()
        .filter_map(|(idx, &g)| if g == 2 { Some(idx) } else { None })
        .collect::<Vec<_>>();
    let mut group3_employees = group
        .iter()
        .enumerate()
        .filter_map(|(idx, &g)| if g == 3 { Some(idx) } else { None })
        .collect::<Vec<_>>();
    group1_employees.sort();
    group2_employees.sort();
    group3_employees.sort();

    let g1_p = match group1_employees.len() {
        0 | 1 => 0,
        l => {
            let mut p = 0;
            for i in 0..(l - 1) {
                for k in (i + 1)..l {
                    let e1 = group1_employees[i];
                    let e2 = group1_employees[k];
                    p += point_list[e1][e2 - 1];
                }
            }
            p
        }
    };
    let g2_p = match group2_employees.len() {
        0 | 1 => 0,
        l => {
            let mut p = 0;
            for i in 0..(l - 1) {
                for k in (i + 1)..l {
                    let e1 = group2_employees[i];
                    let e2 = group2_employees[k];
                    p += point_list[e1][e2 - 1];
                }
            }
            p
        }
    };
    let g3_p = match group3_employees.len() {
        0 | 1 => 0,
        l => {
            let mut p = 0;
            for i in 0..(l - 1) {
                for k in (i + 1)..l {
                    let e1 = group3_employees[i];
                    let e2 = group3_employees[k];
                    p += point_list[e1][e2 - 1];
                }
            }
            p
        }
    };

    g1_p + g2_p + g3_p
}

fn h() {
    let n = read!(usize);
    let mut c = read![[usize]];
    let q = read!(usize);
    let query = read!(String; q);

    let mut set_no_flag = false;
    let mut all_no_flag = false;
    //let mut odd_number = (0..n).filter(|x| x % 2 == 0).map(|x| c[x]).collect::<Vec<_>>();
    //odd_number.sort();
    //odd_number.reverse(); // 降順

    //let mut all_number = c.clone();
    //all_number.sort();
    //all_number.reverse();

    let mut ans = 0;

    for q in query {
        let ch = q.split_whitespace().collect::<Vec<_>>();
        if ch[0] == "1" {
            let x: usize = ch[1].parse().unwrap();
            let a: usize = ch[2].parse().unwrap();
            if c[x - 1] > a {
                // 余剰在庫あり
                c[x - 1] -= a;
                ans += a;
            } else if c[x - 1] == a {
                // ちょうど枯れる
                c[x - 1] = 0;
                all_no_flag = true;
                if x % 2 == 1 {
                    set_no_flag = true;
                }
                ans += a;
            }
        } else if ch[0] == "2" && !set_no_flag {
            let a: usize = ch[1].parse().unwrap();
            let cloned = c.clone();
            let ans_copy = ans;
            //let impossible = (0..n).filter(|x| x % 2 == 0).map(|x| c[x]).any(|x| x < a);
            //if impossible {
            //    continue;
            //}
            for i in (0..n).filter(|x| x % 2 == 0) {
                if c[i] > a {
                    // 余剰在庫あり
                    c[i] -= a;
                    ans += a;
                } else if c[i] == a {
                    // ちょうど枯れる
                    c[i] = 0;
                    all_no_flag = true;
                    set_no_flag = true;
                    ans += a;
                } else {
                    // 無理
                    c = cloned;
                    ans = ans_copy;
                    break;
                }
            }
        } else if ch[0] == "3" && !all_no_flag {
            let a: usize = ch[1].parse().unwrap();
            let cloned = c.clone();
            let ans_copy = ans;
            //let impossible = (0..n).map(|x| c[x]).any(|x| x < a);
            //if impossible {
            //    continue;
            //}
            for i in 0..n {
                if c[i] > a {
                    // 余剰在庫あり
                    c[i] -= a;
                    ans += a;
                } else if c[i] == a {
                    // ちょうど枯れる
                    c[i] = 0;
                    all_no_flag = true;
                    if i % 2 == 0 {
                        set_no_flag = true;
                    }
                    ans += a;
                } else {
                    // impossible
                    c = cloned;
                    ans = ans_copy;
                    break;
                }
            }
        }
    }

    println!("{}", ans);
}

fn main() {
    e();
}

#[test]
fn operation_test() {
    let n_users = 3;
    let base_f = vec![vec!['N'; n_users]; n_users];

    {
        let mut f = base_f.clone();
        // 1 -> 0, 1 -> 2 following
        f[1][0] = 'Y';
        f[1][2] = 'Y';
        // 0 -> 1 following
        f[0][1] = 'Y';

        let op = Operation {
            from: 0,
            to: None,
            op_type: OperationType::Followfollow,
        };

        op.exec(&mut f, n_users);
        assert_eq!(&f, &[['N', 'Y', 'Y'], ['Y', 'N', 'Y'], ['N', 'N', 'N']]);
    }
}
