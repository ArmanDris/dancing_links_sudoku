pub struct Board {
    cells: [i32; 81],
}

impl Default for Board {
    fn default() -> Self {
        Self { cells: [0; 81] }
    }
}

impl Board {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_board(board: &Board) -> Self {
        Self { cells: board.cells }
    }

    pub fn set(&mut self, x: usize, y: usize, value: i32) {
        self.cells[y * 9 + x] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> i32 {
        self.cells[y * 9 + x]
    }

    pub fn get_row(&self, row_idx: usize) -> &[i32] {
        let start = row_idx * 9;
        let end = start + 9;
        &self.cells[start..end]
    }

    pub fn get_column(&self, col_idx: usize) -> [i32; 9] {
        let mut column = [0; 9];

        for (y_idx, slot) in column.iter_mut().enumerate() {
            *slot = self.get(col_idx, y_idx);
        }

        column
    }
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
