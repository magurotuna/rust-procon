#![allow(dead_code)]

pub trait CharToU64 {
    fn to_u64(&self) -> u64;
}

impl CharToU64 for char {
    /// 1, 2, 3, ... の数字の文字型をもとに数値を返す
    fn to_u64(&self) -> u64 {
        *self as u64 - 48
    }
}

/********** 以下テスト **********/

#[test]
fn test_to_u64() {
    assert_eq!('4'.to_u64(), 4);
    assert_eq!('6'.to_u64(), 6);
    assert_eq!('8'.to_u64(), 8);
}
