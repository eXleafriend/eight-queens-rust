fn main() {
    println!("Hello, world!");
}

fn new_empty_board() -> [[bool; 8]; 8] {
    return [
        [false, false, false, false, false, false, false, false,],
        [false, false, false, false, false, false, false, false,],
        [false, false, false, false, false, false, false, false,],
        [false, false, false, false, false, false, false, false,],
        [false, false, false, false, false, false, false, false,],
        [false, false, false, false, false, false, false, false,],
        [false, false, false, false, false, false, false, false,],
        [false, false, false, false, false, false, false, false,],
    ]
}

#[test]
fn test_new_empty_board() {
    let board: [[bool; 8]; 8] = new_empty_board();
    for row in 0..8 {
        for cell in 0..8 {
            assert!(!board[row][cell],
                "Cell value should be false at [{}][{}]", row, cell);
        }
    }
}

fn row_to_string(row: &[bool; 8]) -> String {
    return row.iter().map(|&b| match b {
        false => ".",
        true => "Q",
    })
    .flat_map(|s| s.chars())
    .collect();
}

#[test]
fn test_row_to_string() {
    let f = false;
    let t = true;

    assert_eq!(".Q.Q.Q.Q", row_to_string(&[f, t, f, t, f, t, f, t]));
}

fn board_to_string(board: &[[bool; 8]; 8]) -> String {
    return board.iter().map(|&row|
        row_to_string(&row) + "\n"
    ).collect();
}

#[test]
fn test_board_to_string() {
    let board = new_empty_board();
    let str = board_to_string(&board);

    assert_eq!("".to_string() +
        "........\n" +
        "........\n" +
        "........\n" +
        "........\n" +
        "........\n" +
        "........\n" +
        "........\n" +
        "........\n",
        str);
}

fn offset_to_coordinate(offset: i32) -> (i32, i32) {
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
