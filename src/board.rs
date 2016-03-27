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

    pub fn put(&mut self, offset: usize) {
        let (row, cell) = offset_to_coordinate(offset);
        self.cells[row][cell] = true;
    }

    pub fn remove(&mut self, offset: usize) {
        let (row, cell) = offset_to_coordinate(offset);
        self.cells[row][cell] = false;
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

fn offset_to_coordinate(offset: usize) -> (usize, usize) {
    return (offset / 8, offset % 8);
}

#[test]
fn test_offset_to_coordinate() {
    assert_eq!((0, 0), offset_to_coordinate(0));
    assert_eq!((0, 1), offset_to_coordinate(1));
    assert_eq!((0, 7), offset_to_coordinate(7));
    assert_eq!((1, 0), offset_to_coordinate(8));
    assert_eq!((1, 7), offset_to_coordinate(15));
    assert_eq!((2, 0), offset_to_coordinate(16));
    assert_eq!((7, 7), offset_to_coordinate(63));
}
