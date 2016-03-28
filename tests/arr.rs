extern crate eightqueens;

use self::eightqueens::arr::*;

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

    for _ in arr0 {
    }

}
