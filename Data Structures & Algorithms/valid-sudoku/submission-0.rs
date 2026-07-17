impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows: Vec<HashSet<char>> = vec![HashSet::new(); 9];
    let mut cols: Vec<HashSet<char>> = vec![HashSet::new(); 9];
    let mut squares: Vec<HashSet<char>> = vec![HashSet::new(); 9];

    for row in 0..9 {
        for col in 0..9 {
            let cell = board[row][col];
            if cell != '.' {
                let square_index = (row / 3) * 3 + (col / 3);

                if !rows[row].insert(cell) {
                    return false;
                }

                if !cols[col].insert(cell) {
                    return false;
                }

                if !squares[square_index].insert(cell) {
                    return false;
                }
            }
        }
    }
    true
    }
}
