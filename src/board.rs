use std::ops::{Index, IndexMut};

pub struct Board {
    cells: [[bool; 8]; 8],
}

impl Board {

    pub fn new() -> Board {
        return Board {
            cells: [
                [false, false, false, false, false, false, false, false, ],
                [false, false, false, false, false, false, false, false, ],
                [false, false, false, false, false, false, false, false, ],
                [false, false, false, false, false, false, false, false, ],
                [false, false, false, false, false, false, false, false, ],
                [false, false, false, false, false, false, false, false, ],
                [false, false, false, false, false, false, false, false, ],
                [false, false, false, false, false, false, false, false, ],
            ],
        };
    }

    fn offset_to_coordinate(offset: usize) -> (usize, usize) {
        return (offset / 8, offset % 8);
    }

}

impl Index<usize> for Board {
    type Output = [bool; 8];

    fn index<'a>(&'a self, index: usize) -> &'a [bool; 8] {
        &self.cells[index]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut [bool; 8] {
        &mut self.cells[index]
    }
}


#[test]
fn test_new() {
    let board  = Board::new();
    for row in 0..8 {
        for cell in 0..8 {
            assert!(!board.cells[row][cell],
                "Cell value should be false at [{}][{}]", row, cell);
        }
    }
}

#[test]
fn test_offset_to_coordinate() {
    assert_eq!((0, 0), Board::offset_to_coordinate(0));
    assert_eq!((0, 1), Board::offset_to_coordinate(1));
    assert_eq!((0, 7), Board::offset_to_coordinate(7));
    assert_eq!((1, 0), Board::offset_to_coordinate(8));
    assert_eq!((1, 7), Board::offset_to_coordinate(15));
    assert_eq!((2, 0), Board::offset_to_coordinate(16));
    assert_eq!((7, 7), Board::offset_to_coordinate(63));
}

#[test]
fn test_index() {
    let board  = Board::new();
    assert_eq!(board[0], [false, false, false, false, false, false, false, false, ]);
}

#[test]
fn test_index_mut() {
    let mut board  = Board::new();
    board[1][2] = true;
    assert_eq!(board[1][2], true);
    assert_eq!(board[1], [false, false, true, false, false, false, false, false, ]);
}
