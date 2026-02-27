use crate::board::Board;
use core::panic;
use rand::Rng;
use std::collections::HashSet;

#[cfg(test)]
#[path = "algorithm_x_test.rs"]
mod algorithm_x_tests;

pub struct ConstraintTable {
    pub table: [[bool; 324]; 729],
}

impl Default for ConstraintTable {
    fn default() -> Self {
        Self {
            table: [[false; 324]; 729],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Decision {
    selected_row: usize,
    potential_rows: Vec<usize>,
    rows_conflicting_with_selected_row: Vec<usize>,
}

enum ConstraintType {
    Row,
    Column,
    SubGrid,
    Existence,
}

impl ConstraintType {
    fn get_offset(&self) -> usize {
        match self {
            ConstraintType::Row => 0,
            ConstraintType::Column => 81,
            ConstraintType::SubGrid => 162,
            ConstraintType::Existence => 243,
        }
    }
}

/// This function is responsible for mapping all 324 constraints
/// to a predicable index.
///
/// This writeup:
/// https://web.archive.org/web/20230426084731/https://garethrees.org/2007/06/10/zendoku-generation/#section-4
/// is the inspiration for my method.
///
/// constraint_broad_value is the zero indexed row/column/sub_grid
/// eg. row 1 would be zero, column 5 would be 4.
///
/// constraint_cell_value is the zero indexed cell value
/// eg. row 1 would be zero, column 5 would be 4.
fn map_constraint_to_column_idx(
    constraint_type: ConstraintType,
    constraint_broad_value: usize,
    constraint_cell_value: usize,
) -> usize {
    constraint_type.get_offset() + (constraint_broad_value * 9) + constraint_cell_value
}

/// Generates a row in the constraint table using the choices represented by
/// the board parameter.
/// These constraints take up the first 81 indexes in the constraint table
fn fill_row_constraints(board: &Board, constraint_row: &mut [bool; 324]) {
    for row_idx in 0..9 {
        let board_row = board.get_row(row_idx);
        for cell_idx in 0..9 {
            let required_number = cell_idx as i32 + 1;
            if board_row.contains(&required_number) {
                let index = map_constraint_to_column_idx(ConstraintType::Row, row_idx, cell_idx);
                constraint_row[index] = true;
            }
        }
    }
}

fn fill_column_constraints(board: &Board, constraint_row: &mut [bool; 324]) {
    for column_idx in 0..9 {
        let board_column = board.get_column(column_idx);
        for cell_idx in 0..9 {
            let required_number = cell_idx as i32 + 1;
            if board_column.contains(&required_number) {
                let index =
                    map_constraint_to_column_idx(ConstraintType::Column, column_idx, cell_idx);
                constraint_row[index] = true;
            }
        }
    }
}

/// The sub grid constraints take up index 162 to 243
/// The first nine index'es represent whether the top
/// left sub grid contains each number. The next
/// nine indexes represent whether the top middle sub
/// grid contain each number. Following this pattern
/// the fourth set of nine indexes would represent whether
/// the left middle sub grid has each number.
fn fill_sub_grid_constraints(board: &Board, constraint_row: &mut [bool; 324]) {
    for sub_grid_y_start in (0..9).step_by(3) {
        for sub_grid_x_start in (0..9).step_by(3) {
            let mut sub_grid_numbers: Vec<i32> = vec![];
            // Add row 1
            sub_grid_numbers.push(board.get(sub_grid_y_start, sub_grid_x_start));
            sub_grid_numbers.push(board.get(sub_grid_y_start, sub_grid_x_start + 1));
            sub_grid_numbers.push(board.get(sub_grid_y_start, sub_grid_x_start + 2));
            // Add row 2
            sub_grid_numbers.push(board.get(sub_grid_y_start + 1, sub_grid_x_start));
            sub_grid_numbers.push(board.get(sub_grid_y_start + 1, sub_grid_x_start + 1));
            sub_grid_numbers.push(board.get(sub_grid_y_start + 1, sub_grid_x_start + 2));
            // Add row 3
            sub_grid_numbers.push(board.get(sub_grid_y_start + 2, sub_grid_x_start));
            sub_grid_numbers.push(board.get(sub_grid_y_start + 2, sub_grid_x_start + 1));
            sub_grid_numbers.push(board.get(sub_grid_y_start + 2, sub_grid_x_start + 2));

            for cell_idx in 0..9 {
                let required_number = cell_idx as i32 + 1;
                if sub_grid_numbers.contains(&required_number) {
                    let index = map_constraint_to_column_idx(
                        ConstraintType::SubGrid,
                        sub_grid_x_start + (sub_grid_y_start / 3),
                        cell_idx,
                    );
                    constraint_row[index] = true;
                }
            }
        }
    }
}

fn fill_existence_constraints(board: &Board, constraint_row: &mut [bool; 324]) {
    for row_idx in 0..9 {
        for col_idx in 0..9 {
            let cell_has_value = board.get(col_idx, row_idx) != 0;
            let index = map_constraint_to_column_idx(ConstraintType::Existence, row_idx, col_idx);
            constraint_row[index] = cell_has_value;
        }
    }
}

pub fn fill_constraint_table_row(board: &Board, row: &mut [bool; 324]) {
    fill_row_constraints(board, row);
    fill_column_constraints(board, row);
    fill_sub_grid_constraints(board, row);
    fill_existence_constraints(board, row);
}

pub fn generate_constraint_table() -> ConstraintTable {
    let mut ct = ConstraintTable::default();

    let mut current_constraint_row = 0;

    for row_idx in 0..9 {
        for col_idx in 0..9 {
            for value in 1..10 {
                // Here we need to get the correct row from the constraint table as a slice
                // Then we need to create a board with the specified choice,
                //  - eg for the first index that would be a board with a 1 at 0, 0
                // Then we need to call `fill_constraint_table_row` with the constraint table slice and the board representing the choice
                let constraint_row = &mut ct.table[current_constraint_row];
                let mut board = Board::new();
                board.set(col_idx, row_idx, value);
                fill_constraint_table_row(&board, constraint_row);
                current_constraint_row += 1;
            }
        }
    }

    return ct;
}

fn find_unsatisfied_constraint(
    constraint_table: &[[bool; 324]; 729],
    solution_set: &HashSet<usize>,
) -> Option<usize> {
    if solution_set.len() == 0 {
        return Some(0);
    }

    for col_idx in 0..324 {
        let mut column_satisfied = false;
        for &solution_idx in solution_set {
            let solution_cell = constraint_table[solution_idx][col_idx];
            if solution_cell {
                column_satisfied = true;
                break;
            }
        }

        if !column_satisfied {
            return Some(col_idx);
        }
    }

    None
}

fn find_satisfying_rows(
    ct: &[[bool; 324]; 729],
    hidden_rows: &HashSet<usize>,
    column_to_satisfy: usize,
) -> Vec<usize> {
    let mut rows = vec![];
    for (row_index, row) in ct.iter().enumerate() {
        if hidden_rows.contains(&row_index) {
            continue;
        }

        if row[column_to_satisfy] {
            rows.push(row_index);
        }
    }

    rows
}

fn get_conflicting_rows(
    ct: &[[bool; 324]; 729],
    hidden_row_indexes: &HashSet<usize>,
    selected_row_index: usize,
) -> Vec<usize> {
    let target_row = ct[selected_row_index];

    let mut conflicting_rows = vec![];

    for (constraint_table_row_index, row) in ct.iter().enumerate() {
        if constraint_table_row_index == selected_row_index {
            continue;
        }

        if hidden_row_indexes.contains(&constraint_table_row_index) {
            continue;
        }

        for (column_index, val) in row.iter().enumerate() {
            if *val && target_row[column_index] {
                conflicting_rows.push(constraint_table_row_index);
                break;
            }
        }
    }

    conflicting_rows
}

#[derive(Copy, Clone, Debug)]
pub enum DecisionStrategy {
    First,
    Random,
}
/// REQUIRES: possible_rows is not empty
/// Given an array of possible row_indexes, selects a
/// row defined by strategy, then returns a tuple where
/// the first element is the selected row and the second
/// element is the remaining rows.
fn pick_row(mut possible_rows: Vec<usize>, strategy: DecisionStrategy) -> (usize, Vec<usize>) {
    if possible_rows.is_empty() {
        panic!("Cannot pick row from empty array")
    }

    let selected_row_index = match strategy {
        DecisionStrategy::First => 0,
        DecisionStrategy::Random => rand::thread_rng().gen_range(0..possible_rows.len()),
    };

    let selected_row = possible_rows.swap_remove(selected_row_index);

    (selected_row, possible_rows)
}

/// Returns the last decision in the list that has potential rows
/// Modifies decisions in place, removing all decisions with no
/// potential rows and removing the last decision that has potential
/// rows
/// If it returns none, then that means there are no more decisions to pop
fn get_last_decision(decisions: &mut Vec<Decision>) -> Option<Vec<Decision>> {
    let mut popped_decisions: Vec<Decision> = vec![];
    loop {
        let decision = match decisions.pop() {
            Some(d) => d,
            None => return None,
        };

        let has_potential_rows = !decision.potential_rows.is_empty();

        popped_decisions.push(decision);

        if has_potential_rows {
            break;
        }
    }

    Some(popped_decisions)
}

fn backtrack(
    decisions: &mut Vec<Decision>,
    hidden_rows: &mut HashSet<usize>,
    solution_set: &mut HashSet<usize>,
    decision_strategy: DecisionStrategy,
) -> (usize, Vec<usize>) {
    let mut popped_decisions = match get_last_decision(decisions) {
        Some(popped_ds) => popped_ds,
        None => {
            println!("hidden rows: {:?}", hidden_rows);
            panic!(
                "There are no decisions left to undo, but there is also no possible choice of row. Cannot proceed, exiting"
            )
        }
    };
    for decision in &popped_decisions {
        // Remove selected row from solution set
        solution_set.remove(&decision.selected_row);
        // Remove selected row from hidden rows
        hidden_rows.remove(&decision.selected_row);

        // Unhide the rows that were hidden from this decision
        let conflicting_rows: HashSet<usize> = decision
            .rows_conflicting_with_selected_row
            .iter()
            .copied()
            .collect();
        let _extracted_elements: Vec<_> = hidden_rows
            .extract_if(|v| conflicting_rows.contains(v))
            .collect();
    }

    let popped_decision = match popped_decisions.pop() {
        Some(d) => d,
        None => panic!("If we are here there should have been at least one popped decision"),
    };

    pick_row(popped_decision.potential_rows, decision_strategy)
}

fn map_solution_set_to_board(solution_set: &HashSet<usize>) -> Board {
    // every cell occupies 9 entries inthe table
    // to get the value from an index do: (index % 9) + 1
    // to get the coordinates from an index do (index / 9) = x, where the row = x/9 and the column = x%9
    let mut placements: Vec<(usize, usize, i32)> = vec![];

    for index in solution_set {
        let value: i32 = (index % 9 + 1) as i32;
        let sequential_cell_index = index / 9;
        let cell_x = sequential_cell_index % 9;
        let cell_y = sequential_cell_index / 9;
        placements.push((cell_x, cell_y, value));
    }

    let mut board = Board::new();

    for p in placements {
        board.set(p.0, p.1, p.2);
    }

    board
}

fn map_board_to_solution_set(board: &Board) -> HashSet<usize> {
    // iterate through each cell in the board and get it's corresponding constraint table row
    let mut solution_set: HashSet<usize> = HashSet::new();

    for row_index in 0..9 {
        for column_index in 0..9 {
            let cell = board.get(column_index, row_index);
            if cell == 0 {
                continue;
            }
            let constraint_table_row = (row_index * 9 * 9) + (column_index * 9) + cell as usize - 1;
            solution_set.insert(constraint_table_row);
        }
    }

    solution_set
}

fn generate_initial_state(
    board: Option<Board>,
    constraint_table: &[[bool; 324]; 729],
) -> (HashSet<usize>, HashSet<usize>) {
    let initial_solution_set = match board {
        Some(board) => map_board_to_solution_set(&board),
        None => HashSet::new(),
    };
    let initial_hidden_rows =
        initial_solution_set
            .iter()
            .fold(HashSet::new(), |mut accumulator, row_index| {
                let conflicting_rows =
                    get_conflicting_rows(&constraint_table, &accumulator, *row_index);
                accumulator.extend(conflicting_rows);
                accumulator
            });
    (initial_solution_set, initial_hidden_rows)
}

// TODO: Create public facing functions that clearly
// seperate generating a single board with default params
// vs fine tuning and generating a vector of board
pub fn launch_algorithm_x(
    starting_board: Option<Board>,
    decision_strategy: Option<DecisionStrategy>,
    desired_solutions: Option<usize>,
) -> Vec<Board> {
    // Convert to exact cover problem

    // Constraints:
    //  - all rows must contain 1-9 (81)
    //  - all columns must contain 1-9 (81)
    //  - each subgrid must contain 1-9 (81)
    //  - each cell must contain a value (81)
    // (81 + 81 + 81 + 81) = 324 constraints

    // Choices:
    //  - each cell can be 1-9 (9)
    //  - there are 81 cells (81)
    // 9 * 81 = 729 choices

    let decision_strategy = decision_strategy.unwrap_or(DecisionStrategy::Random);
    let desired_solutions = desired_solutions.unwrap_or(1);
    let mut solutions: Vec<Board> = vec![];

    let constraint_table = generate_constraint_table().table;

    let (initial_solution_set, initial_hidden_rows) =
        generate_initial_state(starting_board, &constraint_table);

    let mut solution_set: HashSet<usize> = initial_solution_set;
    let mut hidden_rows: HashSet<usize> = initial_hidden_rows;

    let mut decisions: Vec<Decision> = vec![];

    loop {
        // Step 1: Pick an unsatisifed constraint
        let unsatisfied_column_idx =
            match find_unsatisfied_constraint(&constraint_table, &solution_set) {
                Some(index) => index,
                None => {
                    solutions.push(map_solution_set_to_board(&solution_set));
                    if solutions.len() >= desired_solutions {
                        return solutions;
                    }
                    backtrack(
                        &mut decisions,
                        &mut hidden_rows,
                        &mut solution_set,
                        decision_strategy,
                    );
                    find_unsatisfied_constraint(&constraint_table, &solution_set).unwrap()
                }
            };

        // Step 2: Get all the rows we can pick to satisfy the constraint
        // If satisfying rows is empty:
        //   1. Pop the last decision. If the popped decision has no
        //      potential_rows we can select, pop another. If we pop
        //      all decisions without finding another potential row
        //      we could have selected. Hard fail, the board is
        //      unsolvable.
        //   2. Iterate through all popped decisions, remove the
        //      selected row from the solution set, and hidden set.
        //      Also remove each decision's conflicting rows from the
        //      hidden rows set.
        //   3. Select the next potential row from the popped decision
        //   4. Calculate all the conflicting rows from the newly picked row
        //      and return those two values as the tuple
        //      (selected_row, possible_rows) to be turned into the next
        //      decision. (possible rows is the calculated conflicting rows)
        let satisfying_rows =
            find_satisfying_rows(&constraint_table, &hidden_rows, unsatisfied_column_idx);

        let (selected_row, possible_rows) = match satisfying_rows.is_empty() {
            false => pick_row(satisfying_rows, decision_strategy),
            true => backtrack(
                &mut decisions,
                &mut hidden_rows,
                &mut solution_set,
                decision_strategy,
            ),
        };

        // Step 3: Add the row to the solution set
        solution_set.insert(selected_row);

        // Step 4: Remove any rows that satisfy any of the constraitns satisfied by the chosen row
        let conflicting_rows = get_conflicting_rows(&constraint_table, &hidden_rows, selected_row);

        hidden_rows.extend(conflicting_rows.iter().copied());

        let decision = Decision {
            selected_row: selected_row,
            potential_rows: possible_rows,
            rows_conflicting_with_selected_row: conflicting_rows,
        };

        decisions.push(decision);
    }
}
