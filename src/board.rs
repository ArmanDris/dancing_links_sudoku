/// Represents a Sudoku board in row-major order. Each entry in the board
/// should be in the range 0..=9, with 0 representing an empty value.
pub struct Board {
    cells: [i32; 81],
}

impl Default for Board {
    /// Returns a board with all cells set to 0 (representing an empty value).
    fn default() -> Self {
        Self { cells: [0; 81] }
    }
}

impl Board {
    /// Returns a board with all cells set to 0 (representing an empty value).
    pub fn new() -> Self {
        Self::default()
    }

    /// Copies `board` into a new instance.
    pub fn from_board(board: &Board) -> Self {
        Self { cells: board.cells }
    }

    /// Sets the value at column `x` and row `y`.
    ///
    /// Both `x` and `y` should be in the range 0..=8
    pub fn set(&mut self, x: usize, y: usize, value: i32) {
        self.cells[y * 9 + x] = value;
    }

    /// Returns the value at column `x` and row `y`.
    ///
    /// Both `x` and `y` should be in the range 0..=8
    pub fn get(&self, x: usize, y: usize) -> i32 {
        self.cells[y * 9 + x]
    }

    /// Returns the `row_idx`'th row. Must be in the range 0..=8
    pub fn get_row(&self, row_idx: usize) -> &[i32] {
        let start = row_idx * 9;
        let end = start + 9;
        &self.cells[start..end]
    }

    /// Returns the `col_idx`'th column. Must be in the range 0..=8
    pub fn get_column(&self, col_idx: usize) -> [i32; 9] {
        let mut column = [0; 9];

        for (y_idx, slot) in column.iter_mut().enumerate() {
            *slot = self.get(col_idx, y_idx);
        }

        column
    }

    /// Prints the sudoku board to stdout
    pub fn print_board(&self) {
        for row in 0..9 {
            let mut row_string = String::from("");
            for cell in 0..9 {
                row_string.push_str(&format!("| {} |", self.get(cell, row)));
            }
            println!("{}", row_string);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sets() {
        let mut board = Board::new();
        board.set(0, 5, 1);
        assert_eq!(board.get(0, 5), 1);
    }

    #[test]
    fn board_get_column() {
        let mut board = Board::new();
        board.set(2, 1, 1);
        board.set(2, 2, 2);
        board.set(2, 3, 5);

        let second_column = board.get_column(2);
        assert_eq!([0, 1, 2, 5, 0, 0, 0, 0, 0], second_column);
    }
}
