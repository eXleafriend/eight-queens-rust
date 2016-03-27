extern crate eightqueens;

use self::eightqueens::arr::*;

#[test]
fn new() {
    let mut arr0;

    arr0 = Arrangement::new(4, 0);
    assert_eq!(arr0.arrangement, vec![false, false, false, false]);

    arr0 = Arrangement::new(4, 1);
    assert_eq!(arr0.arrangement, vec![true, false, false, false]);

    arr0 = Arrangement::new(4, 2);
    assert_eq!(arr0.arrangement, vec![true, true, false, false]);

    arr0 = Arrangement::new(4, 3);
    assert_eq!(arr0.arrangement, vec![true, true, true, false]);

    arr0 = Arrangement::new(4, 4);
    assert_eq!(arr0.arrangement, vec![true, true, true, true]);

}
