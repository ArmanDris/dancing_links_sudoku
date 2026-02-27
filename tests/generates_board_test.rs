use dancing_links_sudoku::Board;
use dancing_links_sudoku::DecisionStrategy;
use dancing_links_sudoku::launch_algorithm_x;

#[test]
fn it_generates_a_board() {
    let solutions = launch_algorithm_x(None, None, None);
    let board = solutions.first().unwrap();

    let mut num_zero = 0;
    for row_idx in 0..9 {
        for col_idx in 0..9 {
            if board.get(col_idx, row_idx) == 0 {
                num_zero += 1;
            }
        }
    }

    assert_eq!(num_zero, 0);
}

#[test]
fn it_generates_many_sequential_boards() {
    const NUM_SOLUTIONS: usize = 10;
    let solutions = launch_algorithm_x(
        Some(Board::new()),
        Some(DecisionStrategy::First),
        Some(NUM_SOLUTIONS),
    );

    assert_eq!(solutions.len(), NUM_SOLUTIONS);
}
