extern crate eightqueens;

use self::eightqueens::board::*;

#[test]
fn new() {
    let board  = Board::new();
    for row in 0..8 {
        for cell in 0..8 {
            assert!(!board[row][cell],
                "Cell value should be false at [{}][{}]", row, cell);
        }
    }
}

#[test]
fn put() {
    let mut board = Board::new();

    board.put(0);
    assert!(board[0][0]);

    board.put(7);
    assert!(board[0][7]);

    board.put(8);
    assert!(board[1][0]);

    board.put(63);
    assert!(board[7][7]);

}

#[test]
fn remove() {
    let mut board = Board::new();

    board.put(0);
    assert!(board[0][0]);
    board.remove(0);
    assert!(!board[0][0]);

    board.put(7);
    assert!(board[0][7]);
    board.remove(7);
    assert!(!board[0][7]);

    board.put(8);
    assert!(board[1][0]);
    board.remove(8);
    assert!(!board[1][0]);

    board.put(63);
    assert!(board[7][7]);
    board.remove(63);
    assert!(!board[7][7]);

}

#[test]
fn index() {
    let board  = Board::new();
    assert_eq!(board[0], [false, false, false, false, false, false, false, false, ]);
}

#[test]
fn index_mut() {
    let mut board  = Board::new();
    board[1][2] = true;
    assert_eq!(board[1][2], true);
    assert_eq!(board[1], [false, false, true, false, false, false, false, false, ]);
}
