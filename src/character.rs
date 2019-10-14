#![allow(dead_code)]

pub trait CharToNum {
    fn to_i32(&self) -> i32;
    fn to_i64(&self) -> i64;
    fn to_u64(&self) -> u64;
    fn to_usize(&self) -> usize;
}

impl CharToNum for char {
    /// 1, 2, 3, ... の数字の文字型をもとにi32を返す
    fn to_i32(&self) -> i32 {
        *self as i32 - 48
    }

    /// 1, 2, 3, ... の数字の文字型をもとにi64を返す
    fn to_i64(&self) -> i64 {
        *self as i64 - 48
    }

    /// 1, 2, 3, ... の数字の文字型をもとにi64を返す
    fn to_u64(&self) -> u64 {
        *self as u64 - 48
    }

    /// 1, 2, 3, ... の数字の文字型をもとにusizeを返す
    fn to_usize(&self) -> usize {
        *self as usize - 48
    }
}

/********** 以下テスト **********/

#[test]
fn test_to_i32() {
    assert_eq!('4'.to_i32(), 4);
    assert_eq!('6'.to_i32(), 6);
    assert_eq!('8'.to_i32(), 8);
}

#[test]
fn test_to_i64() {
    assert_eq!('4'.to_i64(), 4);
    assert_eq!('6'.to_i64(), 6);
    assert_eq!('8'.to_i64(), 8);
}

#[test]
fn test_to_u64() {
    assert_eq!('4'.to_u64(), 4);
    assert_eq!('6'.to_u64(), 6);
    assert_eq!('8'.to_u64(), 8);
}

#[test]
fn test_to_usize() {
    assert_eq!('4'.to_usize(), 4);
    assert_eq!('6'.to_usize(), 6);
    assert_eq!('8'.to_usize(), 8);
}
