extern crate eightqueens;

use self::eightqueens::arr::*;
use std::cmp::Ordering;

#[test]
fn new() {
    let mut arr0;

    arr0 = Arrangement::new(4, 0);
    assert_eq!(arr0.capacity(), 4);
    assert_eq!(arr0.count(), 0);
    assert_eq!(arr0[0], false);
    assert_eq!(arr0[1], false);
    assert_eq!(arr0[2], false);
    assert_eq!(arr0[3], false);

    arr0 = Arrangement::new(4, 1);
    assert_eq!(arr0.capacity(), 4);
    assert_eq!(arr0.count(), 1);
    assert_eq!(arr0[0], true);
    assert_eq!(arr0[1], false);
    assert_eq!(arr0[2], false);
    assert_eq!(arr0[3], false);

    arr0 = Arrangement::new(4, 2);
    assert_eq!(arr0.capacity(), 4);
    assert_eq!(arr0.count(), 2);
    assert_eq!(arr0[0], true);
    assert_eq!(arr0[1], true);
    assert_eq!(arr0[2], false);
    assert_eq!(arr0[3], false);

    arr0 = Arrangement::new(4, 3);
    assert_eq!(arr0.capacity(), 4);
    assert_eq!(arr0.count(), 3);
    assert_eq!(arr0[0], true);
    assert_eq!(arr0[1], true);
    assert_eq!(arr0[2], true);
    assert_eq!(arr0[3], false);

    arr0 = Arrangement::new(4, 4);
    assert_eq!(arr0.capacity(), 4);
    assert_eq!(arr0.count(), 4);
    assert_eq!(arr0[0], true);
    assert_eq!(arr0[1], true);
    assert_eq!(arr0[2], true);
    assert_eq!(arr0[3], true);

    for _ in arr0.iter() {
    }

}

#[test]
fn from() {
    let arr = Arrangement::from(vec![false, true, false]);
    assert_eq!(arr.capacity(), 3);
    assert_eq!(arr.count(), 1);
    assert_eq!(arr[0], false);
    assert_eq!(arr[1], true);
    assert_eq!(arr[2], false);
}

#[test]
fn iter_1_0() {
    let arr = Arrangement::new(1, 0);
    let mut iter = arr.iter();
    assert_eq!(iter.next(), None);
}

#[test]
fn iter_1_1() {
    let arr = Arrangement::new(1, 1);
    let mut iter = arr.iter();
    assert_eq!(iter.next(), None);
}

#[test]
fn iter_2_1() {
    let arr = Arrangement::new(2, 1);
    let mut iter = arr.iter();
    assert_eq!(arr, Arrangement::from(vec![true, false]));
    assert_eq!(iter.next(), Some(Arrangement::from(vec![false, true])));
    assert_eq!(iter.next(), None);
}

#[test]
fn iter_4_2() {
    let arr = Arrangement::new(4, 2);
    let mut iter = arr.iter();
    assert_eq!(arr, Arrangement::from(vec![true, true, false, false]));
    assert_eq!(iter.next(), Some(Arrangement::from(vec![true, false, true, false])));
    assert_eq!(iter.next(), Some(Arrangement::from(vec![false, true, true, false])));
    assert_eq!(iter.next(), Some(Arrangement::from(vec![true, false, false, true])));
    assert_eq!(iter.next(), Some(Arrangement::from(vec![false, true, false, true])));
    assert_eq!(iter.next(), Some(Arrangement::from(vec![false, false, true, true])));
    assert_eq!(iter.next(), None);
}

#[test]
fn partial_cmp() {
    assert!(Arrangement::from(vec![true, true, false, false]) <
        Arrangement::from(vec![true, false, true, false]));
    assert!(Arrangement::from(vec![true, false, true, false]) >
        Arrangement::from(vec![true, true, false, false]));
    assert_eq!(Arrangement::from(vec![true, false, true, false]).partial_cmp(
        &Arrangement::from(vec![true, false, true, false])), Some(Ordering::Equal));
}
