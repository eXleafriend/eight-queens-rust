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
