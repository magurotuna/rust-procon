#![allow(dead_code)]

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

/********** 以下テスト **********/

#[test]
fn test_stepby() {
    assert_eq!(
        vec![3usize, 5, 2, 8, 1]
            .into_iter()
            .stepby(2)
            .collect::<Vec<usize>>(),
        vec![3usize, 2, 1]
    );

    assert_eq!(
        vec![3usize, 5, 2, 8, 1]
            .into_iter()
            .stepby(3)
            .collect::<Vec<usize>>(),
        vec![3usize, 8]
    );
}

#[test]
fn test_vec() {
    assert_eq!(
        vec![3usize, 5, 2, 8, 1].into_iter().step_by(2).vec(),
        vec![3usize, 2, 1]
    );
    assert_eq!("bar".chars().rev().vec(), vec!['r', 'a', 'b']);
}
