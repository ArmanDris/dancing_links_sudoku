use dancing_links_sudoku::{
    DecisionStrategy, advanced_get_sudoku_boards, get_sudoku_boards,
};

#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn it_generates_a_board() {
    let solutions = get_sudoku_boards(1);
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

#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn it_generates_many_sequential_boards() {
    const NUM_SOLUTIONS: usize = 10;
    let solutions = advanced_get_sudoku_boards(
        10,
        DecisionStrategy::First,
        DecisionStrategy::First,
    );

    assert_eq!(solutions.len(), NUM_SOLUTIONS);
}
