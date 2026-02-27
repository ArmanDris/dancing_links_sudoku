use super::*;
use std::collections::HashSet;

#[test]
fn on_an_empty_board_it_returns_the_correct_constraints() {
    let board = Board::new();

    let mut constraint_row = [false; 324];
    fill_row_constraints(&board, &mut constraint_row);

    let all_false = constraint_row
        .iter()
        .all(|element| -> bool { element == &false });

    assert!(all_false);
}

#[test]
fn correctly_resolves_first_row() {
    let mut board = Board::new();
    board.set(0, 0, 1);
    board.set(1, 0, 2);
    board.set(2, 0, 3);
    board.set(3, 0, 4);

    let mut constraint_row = [false; 324];
    fill_row_constraints(&board, &mut constraint_row);

    let first_four_true = &constraint_row[0..4]
        .iter()
        .all(|element| -> bool { element == &true });
    let last_five_false = &constraint_row[4..9]
        .iter()
        .all(|element| -> bool { element == &false });

    assert!(first_four_true);
    assert!(last_five_false);
}

#[test]
fn correctly_resolves_two_rows() {
    let mut board = Board::new();
    board.set(0, 3, 5);
    board.set(4, 3, 6);
    board.set(7, 3, 7);
    board.set(8, 3, 8);

    board.set(0, 5, 1);
    board.set(1, 5, 4);
    board.set(2, 5, 5);
    board.set(3, 5, 8);

    let mut constraint_row = [false; 324];
    fill_row_constraints(&board, &mut constraint_row);

    // fourth row will be from index [3*9, 4*9)
    let fourth_row_section_of_constraints = &constraint_row[27..36];
    let expected_fourth_row = [false, false, false, false, true, true, true, true, false];
    assert_eq!(fourth_row_section_of_constraints, expected_fourth_row);

    let sixth_row_section_of_constraints = &constraint_row[45..54];
    let expected_sixth_row = [true, false, false, true, true, false, false, true, false];
    assert_eq!(sixth_row_section_of_constraints, expected_sixth_row);
}

// ==========
// Test generate_constraint_column
// ==========
#[test]
fn test_no_false_positives() {
    let board = Board::new();
    let mut constraint_column = [false; 324];
    fill_column_constraints(&board, &mut constraint_column);
    let all_false = constraint_column
        .iter()
        .all(|element| -> bool { element == &false });
    assert!(all_false);
}

#[test]
fn test_detects_first_column_constraints() {
    let mut board = Board::new();
    board.set(0, 3, 9);
    board.set(0, 0, 8);
    board.set(0, 8, 1);

    let mut constraint_row = [false; 324];
    fill_column_constraints(&board, &mut constraint_row);

    let offset = ConstraintType::Column.get_offset();
    let first_col_slice = &constraint_row[offset..(offset + 9)];

    let expected_first_column_constraints =
        [true, false, false, false, false, false, false, true, true];
    assert_eq!(first_col_slice, expected_first_column_constraints);
}

#[test]
fn test_detects_two_random_column_constraints() {
    let mut board = Board::new();
    board.set(2, 1, 1);
    board.set(2, 2, 2);
    board.set(2, 3, 5);

    board.set(8, 6, 4);
    board.set(8, 7, 6);
    board.set(8, 8, 7);

    let mut column_constraints = [false; 324];
    fill_column_constraints(&board, &mut column_constraints);

    let column_two_offset = ConstraintType::Column.get_offset() + (9 * 2);
    let column_two_constraints = &column_constraints[column_two_offset..(column_two_offset + 9)];

    let expected_column_two_constraints =
        [true, true, false, false, true, false, false, false, false];
    assert_eq!(column_two_constraints, expected_column_two_constraints);

    let column_eight_offset = ConstraintType::Column.get_offset() + (9 * 8);
    let column_eight_constraints =
        &column_constraints[column_eight_offset..(column_eight_offset + 9)];

    let expected_column_eight_constraints =
        [false, false, false, true, false, true, true, false, false];
    assert_eq!(column_eight_constraints, expected_column_eight_constraints);
}

#[test]
fn test_detects_sub_grid_constraints() {
    let mut board = Board::new();
    // Set the top left sub grid
    board.set(0, 0, 1);
    board.set(1, 1, 2);
    board.set(2, 2, 5);
    board.set(1, 2, 4);
    board.set(2, 0, 9);
    board.set(1, 0, 1);
    // | 1 | 1 | 9 |
    // |   | 2 |   |
    // | 4 |   | 5 |

    // Set the middle subgrid
    board.set(3, 3, 9);
    board.set(4, 3, 5);
    board.set(5, 3, 1);
    board.set(4, 4, 1);
    board.set(5, 4, 3);
    board.set(3, 5, 4);
    // | 9 | 5 | 1 |
    // |   | 1 | 3 |
    // | 4 |   |   |

    // Set the bottom right subgrid
    board.set(6, 6, 2);
    board.set(7, 6, 5);
    board.set(8, 6, 6);
    board.set(6, 7, 8);
    board.set(7, 7, 7);
    board.set(8, 7, 4);
    board.set(6, 8, 3);
    board.set(7, 8, 9);
    board.set(8, 8, 9);
    // | 2 | 5 | 6 |
    // | 8 | 7 | 4 |
    // | 3 | 9 | 9 |

    let top_left_offset = ConstraintType::SubGrid.get_offset();
    let mut sub_grid_constraints = [false; 324];
    fill_sub_grid_constraints(&board, &mut sub_grid_constraints);

    let top_left_constraints = &sub_grid_constraints[top_left_offset..(top_left_offset + 9)];
    let expected_top_left_constraints = [true, true, false, true, true, false, false, false, true];
    assert_eq!(top_left_constraints, expected_top_left_constraints);

    let middle_sub_grid_offset = ConstraintType::SubGrid.get_offset() + (9 * 4);
    let middle_constraints =
        &sub_grid_constraints[middle_sub_grid_offset..(middle_sub_grid_offset + 9)];
    let expected_middle_constraints = [true, false, true, true, true, false, false, false, true];
    assert_eq!(middle_constraints, expected_middle_constraints);

    let bottom_left_offset = ConstraintType::SubGrid.get_offset() + (9 * 8);
    let bottom_left_constraints =
        &sub_grid_constraints[bottom_left_offset..(bottom_left_offset + 9)];
    let expected_bottom_left_constraints = [false, true, true, true, true, true, true, true, true];
    assert_eq!(bottom_left_constraints, expected_bottom_left_constraints);
}

#[test]
fn test_detects_all_constraints_satisfied() {
    let mut board = Board::new();

    // Set row 1
    board.set(0, 0, 3);
    board.set(1, 0, 1);
    board.set(2, 0, 6);
    board.set(3, 0, 5);
    board.set(4, 0, 7);
    board.set(5, 0, 8);
    board.set(6, 0, 4);
    board.set(7, 0, 9);
    board.set(8, 0, 2);
    // Set row 2
    board.set(0, 1, 5);
    board.set(1, 1, 2);
    board.set(2, 1, 9);
    board.set(3, 1, 1);
    board.set(4, 1, 3);
    board.set(5, 1, 4);
    board.set(6, 1, 7);
    board.set(7, 1, 6);
    board.set(8, 1, 8);

    // Set row 3
    board.set(0, 2, 4);
    board.set(1, 2, 8);
    board.set(2, 2, 7);
    board.set(3, 2, 6);
    board.set(4, 2, 2);
    board.set(5, 2, 9);
    board.set(6, 2, 5);
    board.set(7, 2, 3);
    board.set(8, 2, 1);

    // Set row 4
    board.set(0, 3, 2);
    board.set(1, 3, 6);
    board.set(2, 3, 3);
    board.set(3, 3, 4);
    board.set(4, 3, 1);
    board.set(5, 3, 5);
    board.set(6, 3, 9);
    board.set(7, 3, 8);
    board.set(8, 3, 7);

    // Set row 5
    board.set(0, 4, 9);
    board.set(1, 4, 7);
    board.set(2, 4, 4);
    board.set(3, 4, 8);
    board.set(4, 4, 6);
    board.set(5, 4, 3);
    board.set(6, 4, 1);
    board.set(7, 4, 2);
    board.set(8, 4, 5);

    // Set row 6
    board.set(0, 5, 8);
    board.set(1, 5, 5);
    board.set(2, 5, 1);
    board.set(3, 5, 7);
    board.set(4, 5, 9);
    board.set(5, 5, 2);
    board.set(6, 5, 6);
    board.set(7, 5, 4);
    board.set(8, 5, 3);

    // Set row 7
    board.set(0, 6, 1);
    board.set(1, 6, 3);
    board.set(2, 6, 8);
    board.set(3, 6, 9);
    board.set(4, 6, 4);
    board.set(5, 6, 7);
    board.set(6, 6, 2);
    board.set(7, 6, 5);
    board.set(8, 6, 6);

    // Set row 8
    board.set(0, 7, 6);
    board.set(1, 7, 9);
    board.set(2, 7, 2);
    board.set(3, 7, 3);
    board.set(4, 7, 5);
    board.set(5, 7, 1);
    board.set(6, 7, 8);
    board.set(7, 7, 7);
    board.set(8, 7, 4);

    // Set row 9
    board.set(0, 8, 7);
    board.set(1, 8, 4);
    board.set(2, 8, 5);
    board.set(3, 8, 2);
    board.set(4, 8, 8);
    board.set(5, 8, 6);
    board.set(6, 8, 3);
    board.set(7, 8, 1);
    board.set(8, 8, 9);

    let mut constraints_row = [false; 324];
    fill_row_constraints(&board, &mut constraints_row);
    fill_column_constraints(&board, &mut constraints_row);
    fill_sub_grid_constraints(&board, &mut constraints_row);
    fill_existence_constraints(&board, &mut constraints_row);

    let all_true = constraints_row
        .iter()
        .all(|constraint| -> bool { constraint == &true });
    assert!(all_true);
}

#[test]
fn test_generate_constraint_table() {
    // The constraint table's indexes represent this:
    //
    //                | Row 0 has a 1 | Row 0 has a 2 | Row 0 has a 9 | ... | Row 1 has a 1 | ... |
    //
    // Place 1 at 0,0 |     true      |    false      |     false     | ... |    false      | ... |
    // Place 2 at 0,0
    // ...
    // Place 9 at 0,0 |     false     |    false      |      true     | ... |    false      | ... |
    // Place 1 at 1,0 |     true      |    false      |      false    | ... |    false      | ... |
    // ...
    let ct = generate_constraint_table();

    // =====================================
    // For choice of placing 4 at 0,1
    // =====================================
    let first_choice_row = &ct.table[9 * 9 * 1 + 4 - 1];
    // Asserting that the "Row 1 has a 4" constraint is true for choice "Place 4 at 0,1"
    assert!(first_choice_row[9 + 4 - 1]);
    // Asserting that the "Column 0 has a 4" constraint is true for choice "Place 4 at 0,1"
    assert!(first_choice_row[ConstraintType::Column.get_offset() + 4 - 1]);
    // Asserting that the "Subgrid 0 has a 4" constraint is true for choice "Place 4 at 0,1"
    assert!(first_choice_row[ConstraintType::SubGrid.get_offset() + 4 - 1]);
    // Asserting that the "Cell 0,1 contains a value" constraint is true for choice "Place 4 at 0,1"
    assert!(first_choice_row[ConstraintType::Existence.get_offset() + 9]);
    // Asserting that all other constraints are false
    assert!(first_choice_row.iter().filter(|var| **var).count() == 4);

    // =====================================
    // For choice of placing 7 at 8, 4
    // =====================================
    let second_choice_row = &ct.table[(9 * 8) + (9 * 9 * 4) + 7 - 1];
    // Asserting that the "Row 4 has a 7" constraint is true for choice "Place 7 at 8, 4"
    assert!(second_choice_row[(9 * 4) + 7 - 1]);
    // Asserting that the "Column 8 has a 7" constraint is true for choice "Place 7 at 8,4"
    assert!(second_choice_row[ConstraintType::Column.get_offset() + (9 * 8) + 7 - 1]);
    // Asserting that the "Subgrid 5 has a 7" constraint is true for choice "Place 7 at 8,4"
    assert!(second_choice_row[ConstraintType::SubGrid.get_offset() + (9 * 5) + 7 - 1]);
    // Asserting that the "Cell 8,4 has a value" constraint is true for choice "Place 7 at 8,4"
    assert!(second_choice_row[ConstraintType::Existence.get_offset() + (4 * 9) + 8]);
    // Asserting that all other constraints are false
    assert!(second_choice_row.iter().filter(|var| **var).count() == 4);

    // =====================================
    // For choice of placting 1 at 3,0
    // =====================================
    let third_choice_row = &ct.table[(9 * 3) + 1 - 1];
    // Assert that the "Row 0 has a 1" constraint is true for "Place 1 at 3,0"
    assert!(third_choice_row[1 - 1]);
    // Assert that the "Column 3 has a 1" constraint is true for "Place 1 at 3,0"
    assert!(third_choice_row[ConstraintType::Column.get_offset() + (9 * 3) + 1 - 1]);
    // Assert that the "Subgrid 1 has a 1" constraint is true for "Place 1 at 3,0"
    assert!(third_choice_row[ConstraintType::SubGrid.get_offset() + 9 + 1 - 1]);
    // Assert that all other constraints are false
    assert!(third_choice_row.iter().filter(|var| **var).count() == 4);
}

#[test]
fn test_find_satisfying_row_scenarios() {
    // 1) No row satisfies the column → None
    let mut ct = [[false; 324]; 729];
    let col = 137;
    let hidden = HashSet::<usize>::new();
    assert_eq!(find_satisfying_rows(&ct, &hidden, col), vec![]);

    // 2) A single row satisfies → that row index
    ct[10][col] = true;
    assert_eq!(find_satisfying_rows(&ct, &hidden, col), vec![10]);

    // 3) An earlier row also satisfies, but it’s hidden → return next visible (10)
    ct[3][col] = true;
    let mut hidden = HashSet::new();
    hidden.insert(3);
    assert_eq!(find_satisfying_rows(&ct, &hidden, col), vec![10]);

    // 4) If earlier row isn’t hidden → earliest match (3) wins
    let hidden = HashSet::<usize>::new();
    assert_eq!(find_satisfying_rows(&ct, &hidden, col).first(), Some(&3));

    // 5) Hide both earlier matches; ensure it finds a later satisfying row
    let mut hidden = HashSet::new();
    hidden.insert(3);
    hidden.insert(10);
    ct[400][col] = true;
    assert_eq!(find_satisfying_rows(&ct, &hidden, col).first(), Some(&400));

    // 6) Boundary columns: 0 and 241
    let mut ct2 = [[false; 324]; 729];

    ct2[5][0] = true;
    assert_eq!(
        find_satisfying_rows(&ct2, &HashSet::new(), 0).first(),
        Some(&5)
    );

    ct2[5][0] = false;
    ct2[25][241] = true;
    assert_eq!(
        find_satisfying_rows(&ct2, &HashSet::new(), 241).first(),
        Some(&25)
    );
}

#[test]
fn test_get_conflicting_rows() {
    const N_COLS: usize = 324;
    const N_ROWS: usize = 729;

    fn row_with(cols_true: &[usize]) -> [bool; N_COLS] {
        let mut row = [false; N_COLS];
        for &c in cols_true {
            row[c] = true;
        }
        row
    }

    // Build a full constraint table with a few meaningful rows, rest all-false.
    let mut ct = [[false; N_COLS]; N_ROWS];

    // Selected row: true at columns 0, 5, 241
    ct[10] = row_with(&[0, 5, 241]);

    // Conflicting rows (share at least one true col with row 10)
    ct[3] = row_with(&[5]); // shares 5
    ct[7] = row_with(&[241]); // shares 241
    ct[15] = row_with(&[0, 100]); // shares 0 (and some other)
    ct[20] = row_with(&[0, 5, 241]); // shares many

    // Non-conflicting rows
    ct[8] = row_with(&[1, 2, 3]); // no overlap
    ct[9] = row_with(&[]); // all false
    ct[11] = row_with(&[6, 7]); // no overlap with 0,5,241

    // Hidden rows: one conflicting row is hidden, one non-conflicting row hidden too
    let mut hidden = HashSet::new();
    hidden.insert(7); // would conflict, but hidden -> should be excluded
    hidden.insert(8); // doesn't conflict anyway

    // Act
    let mut got = get_conflicting_rows(&ct, &hidden, 10);
    got.sort_unstable();

    // Expect: conflicting rows except selected (10) and hidden (7)
    let mut expected = vec![3, 15, 20];
    expected.sort_unstable();
    assert_eq!(got, expected);

    // Additional check: if we hide nothing, we should get 7 back too
    let empty_hidden = HashSet::new();
    let mut got2 = get_conflicting_rows(&ct, &empty_hidden, 10);
    got2.sort_unstable();

    let mut expected2 = vec![3, 7, 15, 20];
    expected2.sort_unstable();
    assert_eq!(got2, expected2);

    // Additional check: selecting a row with no trues should yield no conflicts
    // (since condition requires *val && target_row[column_index])
    let got3 = get_conflicting_rows(&ct, &empty_hidden, 9);
    assert!(got3.is_empty());
}

#[test]
fn test_pick_row() {
    // TODO: Assert that this panicks for an empty input
    // Empty input -> Panic (both strategies)

    // Single element -> always that element, remainder empty (both strategies)
    let r = pick_row(vec![42], DecisionStrategy::First);
    assert_eq!(r.0, 42);
    assert!(r.1.is_empty());

    let r = pick_row(vec![42], DecisionStrategy::Random);
    assert_eq!(r.0, 42);
    assert!(r.1.is_empty());

    // Multiple elements, First strategy: selects index 0, remainder is original without that element
    let input = vec![10, 20, 30, 40];
    let (selected, remaining) = pick_row(input.clone(), DecisionStrategy::First);
    assert_eq!(selected, 10);
    assert_eq!(remaining.len(), input.len() - 1);

    // Remaining should contain all originals except selected
    let mut remaining_set: HashSet<usize> = remaining.into_iter().collect();
    assert_eq!(remaining_set.len(), 3); // no duplicates introduced
    assert!(remaining_set.remove(&20));
    assert!(remaining_set.remove(&30));
    assert!(remaining_set.remove(&40));
    assert!(remaining_set.is_empty());

    // Multiple elements, Random strategy:
    // We can't assert which element was picked, but we can assert invariants.
    let input = vec![1, 2, 3, 4, 5];
    let (selected, remaining) = pick_row(input.clone(), DecisionStrategy::Random);
    assert_eq!(remaining.len(), input.len() - 1);
    assert!(input.contains(&selected));

    // Selected + remaining should equal the original multiset (no loss/duplication)
    let mut all: HashSet<usize> = remaining.into_iter().collect();
    assert!(all.insert(selected)); // selected shouldn't already be in remaining
    let input_set: HashSet<usize> = input.into_iter().collect();
    assert_eq!(all, input_set);
}

#[test]
fn test_backtracking() {
    let mut decisions: Vec<Decision> = vec![
        Decision {
            selected_row: 0,
            potential_rows: vec![1, 2, 3],
            rows_conflicting_with_selected_row: vec![4, 5, 6],
        },
        Decision {
            selected_row: 7,
            potential_rows: vec![],
            rows_conflicting_with_selected_row: vec![8, 9, 10],
        },
        Decision {
            selected_row: 11,
            potential_rows: vec![12],
            rows_conflicting_with_selected_row: vec![13, 14, 15],
        },
    ];
    let mut hidden_rows: HashSet<usize> = HashSet::from([4, 5, 6, 8, 9, 10, 13, 14, 15]);
    let mut solution_set: HashSet<usize> = HashSet::from([0, 7, 11]);

    let (selected_row, potential_rows) = backtrack(
        &mut decisions,
        &mut hidden_rows,
        &mut solution_set,
        DecisionStrategy::First,
    );

    assert_eq!(selected_row, 12);
    assert_eq!(potential_rows, vec![]);
    assert_eq!(
        decisions,
        vec![
            Decision {
                selected_row: 0,
                potential_rows: vec![1, 2, 3],
                rows_conflicting_with_selected_row: vec![4, 5, 6]
            },
            Decision {
                selected_row: 7,
                potential_rows: vec![],
                rows_conflicting_with_selected_row: vec![8, 9, 10]
            },
        ]
    );
    assert_eq!(hidden_rows, HashSet::from([4, 5, 6, 8, 9, 10]));
    assert_eq!(solution_set, HashSet::from([0, 7]));

    let (selected_row, potential_rows) = backtrack(
        &mut decisions,
        &mut hidden_rows,
        &mut solution_set,
        DecisionStrategy::First,
    );
    assert_eq!(selected_row, 1);
    assert_eq!(potential_rows, vec![3, 2]);
    assert_eq!(decisions, vec![]);
    assert_eq!(hidden_rows, HashSet::new());
    assert_eq!(solution_set, HashSet::new());
}

#[test]
fn test_map_solution_set_to_board() {
    // we want the solution set to look like this:
    // 1 |   |   |
    // - | - | - | -
    //   |   |   | 4
    // - | - | - | -
    // 5 |   |   |
    // - | - | - | -
    //   |   |   | 9
    // ie. (0, 0, 1), (3, 1, 4), (0, 2, 5), (3, 3, 9)
    // This corresponds to constraint table rows:
    // - 0
    // - (9 * 9 * 1) + (9 * 3) + 4 - 1 = 111
    // - (9 * 9 * 2) + 5 - 1 = 166
    // - (9 * 9 * 3) + (9 * 3) + 9 - 1 = 278
    let solution_set = HashSet::from([0, 111, 166, 278]);
    let board = map_solution_set_to_board(&solution_set);
    board.print_board();
    assert!(board.get(0, 0) == 1);
    assert!(board.get(3, 1) == 4);
    assert!(board.get(0, 2) == 5);
    assert!(board.get(3, 3) == 9);

    let mut non_zero_count = 0;

    for row_idx in 0..9 {
        for col_idx in 0..9 {
            if board.get(col_idx, row_idx) != 0 {
                non_zero_count += 1;
            }
        }
    }

    assert_eq!(non_zero_count, 4);
}

#[test]
fn test_generate_initial_state() {
    // This was tested by first running the algorithm
    // organically through 150 iterations, then
    // recording the board state and the algorithm state.
    //
    // If the function can generate the same algorithm state
    // from the board state, then the function works

    let mut board = Board::new();
    // Set column 0
    board.set(0, 0, 1);
    board.set(0, 1, 7);
    board.set(0, 2, 4);
    board.set(0, 3, 2);
    board.set(0, 4, 3);
    board.set(0, 5, 0);
    board.set(0, 6, 0);
    board.set(0, 7, 0);
    board.set(0, 8, 0);
    // Set column 1
    board.set(1, 0, 2);
    board.set(1, 1, 8);
    board.set(1, 2, 5);
    board.set(1, 3, 1);
    board.set(1, 4, 0);
    board.set(1, 5, 0);
    board.set(1, 6, 0);
    board.set(1, 7, 0);
    board.set(1, 8, 0);
    // Set column 2
    board.set(2, 0, 3);
    board.set(2, 1, 9);
    board.set(2, 2, 6);
    board.set(2, 3, 4);
    board.set(2, 4, 5);
    board.set(2, 5, 0);
    board.set(2, 6, 0);
    board.set(2, 7, 0);
    board.set(2, 8, 0);
    // Set column 3
    board.set(3, 0, 4);
    board.set(3, 1, 1);
    board.set(3, 2, 7);
    board.set(3, 3, 3);
    board.set(3, 4, 2);
    board.set(3, 5, 0);
    board.set(3, 6, 0);
    board.set(3, 7, 0);
    board.set(3, 8, 0);
    // Set column 4
    board.set(4, 0, 5);
    board.set(4, 1, 2);
    board.set(4, 2, 8);
    board.set(4, 3, 6);
    board.set(4, 4, 1);
    board.set(4, 5, 0);
    board.set(4, 6, 0);
    board.set(4, 7, 0);
    board.set(4, 8, 0);
    // Set column 5
    board.set(5, 0, 6);
    board.set(5, 1, 3);
    board.set(5, 2, 9);
    board.set(5, 3, 5);
    board.set(5, 4, 0);
    board.set(5, 5, 0);
    board.set(5, 6, 0);
    board.set(5, 7, 0);
    board.set(5, 8, 0);
    // Set column 6
    board.set(6, 0, 7);
    board.set(6, 1, 6);
    board.set(6, 2, 1);
    board.set(6, 3, 9);
    board.set(6, 4, 0);
    board.set(6, 5, 0);
    board.set(6, 6, 0);
    board.set(6, 7, 0);
    board.set(6, 8, 0);
    // Set column 7
    board.set(7, 0, 8);
    board.set(7, 1, 5);
    board.set(7, 2, 2);
    board.set(7, 3, 7);
    board.set(7, 4, 4);
    board.set(7, 5, 0);
    board.set(7, 6, 0);
    board.set(7, 7, 0);
    board.set(7, 8, 0);
    // Set column 8
    board.set(8, 0, 9);
    board.set(8, 1, 4);
    board.set(8, 2, 3);
    board.set(8, 3, 8);
    board.set(8, 4, 6);
    board.set(8, 5, 0);
    board.set(8, 6, 0);
    board.set(8, 7, 0);
    board.set(8, 8, 0);

    let generated_solution_set = map_board_to_solution_set(&board);
    assert_eq!(
        generated_solution_set,
        HashSet::from([
            107, 305, 128, 360, 118, 205, 401, 236, 87, 80, 264, 226, 10, 70, 148, 50, 156, 140,
            216, 252, 346, 97, 20, 215, 175, 30, 108, 185, 60, 272, 165, 322, 352, 326, 390, 292,
            40, 244, 195, 284, 312, 0
        ])
    );

    let constraint_table = generate_constraint_table().table;

    let generated_hidden_rows =
        generated_solution_set
            .iter()
            .fold(HashSet::new(), |mut accumulator, row_index| {
                let conflicting_rows =
                    get_conflicting_rows(&constraint_table, &accumulator, *row_index);
                accumulator.extend(conflicting_rows);
                accumulator
            });

    assert_eq!(
        generated_hidden_rows,
        HashSet::from([
            432, 29, 596, 278, 633, 398, 247, 492, 34, 306, 211, 338, 75, 476, 26, 184, 92, 196,
            684, 499, 676, 577, 327, 359, 363, 697, 489, 425, 4, 361, 658, 228, 396, 255, 418, 378,
            111, 114, 210, 671, 485, 290, 275, 529, 366, 11, 441, 288, 162, 267, 595, 329, 8, 274,
            723, 459, 136, 251, 427, 235, 539, 455, 385, 367, 552, 121, 402, 315, 458, 42, 266, 39,
            132, 707, 616, 129, 349, 380, 190, 523, 311, 382, 56, 442, 509, 405, 257, 393, 89, 403,
            119, 634, 52, 207, 57, 695, 641, 253, 32, 546, 364, 356, 636, 443, 590, 53, 117, 137,
            282, 212, 610, 76, 436, 220, 152, 664, 84, 454, 677, 516, 71, 113, 421, 462, 59, 74,
            238, 553, 287, 362, 17, 482, 12, 246, 109, 139, 388, 19, 567, 303, 151, 90, 506, 145,
            573, 570, 434, 295, 88, 300, 452, 342, 373, 689, 27, 314, 678, 654, 55, 383, 717, 276,
            263, 431, 626, 45, 78, 104, 642, 54, 100, 112, 219, 103, 221, 213, 188, 245, 150, 583,
            670, 249, 68, 391, 576, 519, 535, 258, 158, 394, 293, 384, 313, 168, 317, 72, 438, 386,
            101, 445, 13, 110, 323, 187, 149, 631, 146, 28, 301, 6, 270, 357, 16, 234, 231, 600,
            372, 85, 513, 155, 484, 649, 472, 153, 202, 254, 354, 728, 340, 637, 138, 483, 157,
            669, 608, 161, 561, 681, 644, 243, 91, 347, 351, 343, 299, 120, 548, 397, 614, 374,
            487, 550, 469, 515, 522, 514, 344, 83, 597, 224, 332, 166, 125, 540, 381, 467, 399,
            309, 63, 18, 122, 471, 387, 594, 714, 480, 179, 433, 48, 556, 406, 73, 7, 289, 201,
            651, 414, 486, 256, 404, 49, 98, 331, 533, 66, 62, 159, 222, 334, 426, 163, 508, 144,
            675, 135, 423, 58, 698, 512, 192, 61, 281, 265, 727, 336, 674, 328, 1, 198, 99, 229,
            408, 93, 350, 174, 134, 116, 147, 569, 194, 371, 102, 330, 368, 67, 232, 428, 37, 589,
            2, 131, 22, 31, 79, 286, 241, 563, 106, 46, 164, 181, 603, 280, 44, 320, 277, 127, 718,
            657, 248, 648, 588, 474, 337, 379, 172, 333, 646, 218, 262, 64, 607, 502, 171, 23, 464,
            446, 369, 173, 725, 593, 587, 230, 691, 41, 496, 617, 3, 715, 279, 450, 395, 479, 297,
            465, 204, 142, 35, 9, 495, 269, 209, 566, 712, 21, 400, 177, 273, 176, 160, 123, 186,
            415, 82, 86, 661, 217, 214, 560, 197, 298, 291, 365, 650, 555, 437, 77, 407, 95, 193,
            124, 51, 417, 259, 182, 302, 304, 668, 94, 318, 527, 261, 271, 14, 701, 199, 488, 36,
            702, 283, 296, 451, 627, 688, 377, 621, 69, 180, 96, 237, 409, 141, 189, 169, 200, 310,
            389, 448, 370, 466, 208, 308, 629, 545, 24, 115, 321, 565, 353, 227, 507, 203, 416,
            647, 206, 65, 604, 335, 536, 710, 435, 15, 294, 355, 475, 183, 580, 392, 47, 722, 154,
            191, 126, 223, 268, 239, 170, 319, 81, 260, 348, 242, 130, 233, 324, 250, 411, 133,
            307, 325, 473, 316, 240, 358, 38, 43, 143, 167, 33, 708, 105, 620, 25, 285, 526, 568,
            5, 178, 424, 345, 225, 685
        ])
    );
}

#[test]
fn test_no_unnecessary_backtracks() {
    let mut board = Board::new();
    // Set column 0
    board.set(0, 0, 1);
    board.set(0, 1, 7);
    board.set(0, 2, 4);
    board.set(0, 3, 2);
    board.set(0, 4, 8);
    board.set(0, 5, 3);
    board.set(0, 6, 6);
    board.set(0, 7, 5);
    board.set(0, 8, 0);
    // Set column 1
    board.set(1, 0, 2);
    board.set(1, 1, 8);
    board.set(1, 2, 5);
    board.set(1, 3, 1);
    board.set(1, 4, 9);
    board.set(1, 5, 6);
    board.set(1, 6, 7);
    board.set(1, 7, 4);
    board.set(1, 8, 3);
    // Set column 2
    board.set(2, 0, 3);
    board.set(2, 1, 9);
    board.set(2, 2, 6);
    board.set(2, 3, 4);
    board.set(2, 4, 7);
    board.set(2, 5, 5);
    board.set(2, 6, 1);
    board.set(2, 7, 8);
    board.set(2, 8, 2);
    // Set column 3
    board.set(3, 0, 4);
    board.set(3, 1, 1);
    board.set(3, 2, 7);
    board.set(3, 3, 3);
    board.set(3, 4, 2);
    board.set(3, 5, 9);
    board.set(3, 6, 8);
    board.set(3, 7, 6);
    board.set(3, 8, 0);
    // Set column 4
    board.set(4, 0, 5);
    board.set(4, 1, 2);
    board.set(4, 2, 8);
    board.set(4, 3, 6);
    board.set(4, 4, 1);
    board.set(4, 5, 7);
    board.set(4, 6, 9);
    board.set(4, 7, 3);
    board.set(4, 8, 0);
    // Set column 5
    board.set(5, 0, 6);
    board.set(5, 1, 3);
    board.set(5, 2, 9);
    board.set(5, 3, 5);
    board.set(5, 4, 4);
    board.set(5, 5, 8);
    board.set(5, 6, 2);
    board.set(5, 7, 7);
    board.set(5, 8, 1);
    // Set column 6
    board.set(6, 0, 7);
    board.set(6, 1, 6);
    board.set(6, 2, 1);
    board.set(6, 3, 9);
    board.set(6, 4, 5);
    board.set(6, 5, 4);
    board.set(6, 6, 3);
    board.set(6, 7, 2);
    board.set(6, 8, 0);
    // Set column 7
    board.set(7, 0, 8);
    board.set(7, 1, 5);
    board.set(7, 2, 2);
    board.set(7, 3, 7);
    board.set(7, 4, 3);
    board.set(7, 5, 1);
    board.set(7, 6, 4);
    board.set(7, 7, 9);
    board.set(7, 8, 0);
    // Set column 8
    board.set(8, 0, 9);
    board.set(8, 1, 4);
    board.set(8, 2, 3);
    board.set(8, 3, 8);
    board.set(8, 4, 6);
    board.set(8, 5, 2);
    board.set(8, 6, 5);
    board.set(8, 7, 1);
    board.set(8, 8, 0);

    let solutions = launch_algorithm_x(Some(board), None, Some(1));
    let solution = solutions.first().unwrap();

    let mut zero_exists = false;
    for row_idx in 0..9 {
        zero_exists = zero_exists || solution.get_row(row_idx).iter().any(|e| e == &0);
    }
    assert_eq!(zero_exists, false);
}

#[test]
fn test_algorithm_x() {
    let solutions = launch_algorithm_x(None, None, Some(1));
    let solution = solutions.first().unwrap();

    let mut zero_exists = false;
    for row_idx in 0..9 {
        zero_exists = zero_exists || solution.get_row(row_idx).iter().any(|e| e == &0);
    }
    assert_eq!(zero_exists, false);
}

// // Below will run 10 benchmarks
// #[test]
// fn test_arm() {
//   const RUNS: usize = 10;

//   let mut total = Duration::ZERO;

//   for i in 0..RUNS {
//     let start = Instant::now();
//     let _solutions = launch_algorithm_x(None, None, Some(100));
//     let elapsed = start.elapsed();

//     println!("Run {} took {:?}", i + 1, elapsed);
//     total += elapsed;
//   }

//   let average = total / RUNS as u32;
//   println!("\nAverage runtime over {} runs: {:?}", RUNS, average);

//   assert!(false);
// }
