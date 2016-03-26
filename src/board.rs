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
