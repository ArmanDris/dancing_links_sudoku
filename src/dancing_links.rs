use rand::Rng;
use std::{array, collections::HashSet};

use crate::Board;

use crate::algorithm_x::{
    ConstraintTable, generate_constraint_table, map_solution_set_to_board,
};

#[cfg(test)]
#[path = "dancing_links_test.rs"]
mod dancing_links_test;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Cell {
    column_index: usize,
    row_index: usize,
    up: Option<usize>,
    down: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ColumnHeader {
    cell_count: i32,
    up: Option<usize>,
    down: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Link {
    ColumnHeader(ColumnHeader),
    Cell(Cell),
    EmptyLink,
}

const LINKED_TABLE_COLUMNS: usize = 324;
const LINKED_TABLE_ROWS: usize = 730;

struct LinkedTable {
    table: Box<[[Link; LINKED_TABLE_COLUMNS]; LINKED_TABLE_ROWS]>,
}
/// Strategy to select the next column in Dancing Links search
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecisionStrategy {
    First,
    Random,
    Optimal,
}

struct Decision {
    selected_column: usize,
    selected_row: usize,
    potential_rows: Vec<usize>,
    hidden_columns: Vec<usize>,
}

impl Default for LinkedTable {
    fn default() -> Self {
        let mut vec_table = vec![];

        for _index in 0..LINKED_TABLE_ROWS {
            vec_table.push([Link::EmptyLink; LINKED_TABLE_COLUMNS]);
        }

        let typed_boxed_table: Box<
            [[Link; LINKED_TABLE_COLUMNS]; LINKED_TABLE_ROWS],
        > = match vec_table.into_boxed_slice().try_into() {
            Ok(result) => result,
            Err(_err) => {
                panic!("unable to initialize empty dancing link table")
            }
        };

        let constraint_table = generate_constraint_table();
        let mut linked_table = Self {
            table: typed_boxed_table,
        };

        linked_table.table[0] = generate_column_headers(&constraint_table);
        linked_table.table[1..]
            .clone_from_slice(&*generate_unlinked_rows(&constraint_table));

        linked_table
    }
}

/// Generates a row of column headers with the correct
/// cell counts. DOES NOT INITIALIZE UP, DOWN, LEFT
/// RIGHT POINTERS, THOSE ARE LEFT AS `None`
fn generate_column_headers(constraint_table: &ConstraintTable) -> [Link; 324] {
    let column_cell_counts = array::from_fn::<i32, 324, _>(|row_index| {
        let mut count = 0;
        for column_index in 0..729 {
            if constraint_table.table[column_index][row_index] {
                count += 1;
            }
        }

        count
    });

    array::from_fn::<Link, 324, _>(|row_index| {
        Link::ColumnHeader(ColumnHeader {
            cell_count: column_cell_counts[row_index],
            up: None,
            down: None,
            left: None,
            right: None,
        })
    })
}

fn generate_unlinked_rows(
    constraint_table: &ConstraintTable,
) -> Box<[[Link; 324]; 729]> {
    let mut linked_rows: Vec<[Link; 324]> = vec![];

    for (row_idx, row) in constraint_table.table.iter().enumerate() {
        let mut current_linked = [Link::EmptyLink; 324];
        for (col_idx, cell) in row.iter().enumerate() {
            if !cell {
                continue;
            }

            current_linked[col_idx] = Link::Cell(Cell {
                column_index: col_idx,
                row_index: row_idx,
                up: None,
                down: None,
                left: None,
                right: None,
            })
        }
        linked_rows.push(current_linked);
    }

    let linked_arm: Box<[[Link; 324]; 729]> = linked_rows.try_into().unwrap();

    linked_arm
}

fn link_unlinked_table(linked_table: &mut LinkedTable) -> () {
    // This for loop will link each row together, then each column together

    let table = &mut linked_table.table;
    for row_index in 0..730 {
        let mut first_link_index: Option<usize> = None;
        let mut last_link_index: Option<usize> = None;

        for column_index in 0..324 {
            if table[row_index][column_index] == Link::EmptyLink {
                continue;
            }

            // If this is the first Link, init first and last index pointers
            // and skip to next iteration, otherwise continue.
            match first_link_index {
                Some(_) => (),
                None => {
                    first_link_index = Some(column_index);
                    last_link_index = Some(column_index);
                    continue;
                }
            };

            match &mut table[row_index][column_index] {
                Link::EmptyLink => (),
                Link::ColumnHeader(column_header) => {
                    column_header.left = last_link_index;
                }
                Link::Cell(cell) => {
                    cell.left = last_link_index;
                }
            };

            match &mut table[row_index][last_link_index.unwrap()] {
                Link::EmptyLink => (),
                Link::ColumnHeader(column_header) => {
                    column_header.right = Some(column_index)
                }
                Link::Cell(cell) => cell.right = Some(column_index),
            }

            last_link_index = Some(column_index);
        }

        if first_link_index.is_none() || last_link_index.is_none() {
            panic!(
                "first_link_index was never initialized in link_linked_table, this is a bad state, hard failing",
            );
        }
        let first_link_index = first_link_index.unwrap();
        let last_link_index = last_link_index.unwrap();

        match &mut table[row_index][first_link_index] {
            Link::EmptyLink => (),
            Link::ColumnHeader(column_header) => {
                column_header.left = Some(last_link_index)
            }
            Link::Cell(cell) => cell.left = Some(last_link_index),
        };

        match &mut table[row_index][last_link_index] {
            Link::EmptyLink => (),
            Link::ColumnHeader(column_header) => {
                column_header.right = Some(first_link_index)
            }
            Link::Cell(cell) => cell.right = Some(first_link_index),
        };
    }

    // Linking each column's cells together
    for column_index in 0..LINKED_TABLE_COLUMNS {
        let mut first_link_index: Option<usize> = None;
        let mut last_link_index: Option<usize> = None;

        for row_index in 0..LINKED_TABLE_ROWS {
            if table[row_index][column_index] == Link::EmptyLink {
                continue;
            }

            match first_link_index {
                Some(_) => (),
                None => {
                    first_link_index = Some(row_index);
                    last_link_index = Some(row_index);
                    continue;
                }
            };

            match &mut table[row_index][column_index] {
                Link::EmptyLink => (),
                Link::ColumnHeader(column_header) => {
                    column_header.up = last_link_index
                }
                Link::Cell(cell) => cell.up = last_link_index,
            };

            match &mut table[last_link_index.unwrap()][column_index] {
                Link::EmptyLink => (),
                Link::ColumnHeader(column_header) => {
                    column_header.down = Some(row_index)
                }
                Link::Cell(cell) => cell.down = Some(row_index),
            };

            last_link_index = Some(row_index);
        }

        if first_link_index.is_none() || last_link_index.is_none() {
            panic!(
                "first_link_index, or last_link_index was never initialized, this is a bad start, aborting table creation"
            );
        }
        let first_link_index = first_link_index.unwrap();
        let last_link_index = last_link_index.unwrap();

        match &mut table[first_link_index][column_index] {
            Link::EmptyLink => (),
            Link::ColumnHeader(column_header) => {
                column_header.up = Some(last_link_index)
            }
            Link::Cell(cell) => cell.up = Some(last_link_index),
        };

        match &mut table[last_link_index][column_index] {
            Link::EmptyLink => (),
            Link::ColumnHeader(column_header) => {
                column_header.down = Some(first_link_index)
            }
            Link::Cell(cell) => cell.down = Some(first_link_index),
        };
    }
}

/// Selects an active column according to the requested strategy.
fn select_column(
    active_columns: &[bool; LINKED_TABLE_COLUMNS],
    decision_strategy: DecisionStrategy,
    table: &LinkedTable,
) -> usize {
    if !active_columns.iter().any(|is_active| *is_active) {
        panic!("No columns to select");
    }

    match decision_strategy {
        DecisionStrategy::First => active_columns
            .iter()
            .enumerate()
            .find_map(
                |(column_idx, is_active)| {
                    if *is_active { Some(column_idx) } else { None }
                },
            )
            .unwrap(),
        DecisionStrategy::Random => {
            let potential_columns: Vec<usize> =
                active_columns
                    .iter()
                    .enumerate()
                    .filter_map(|(column_idx, is_active)| {
                        if *is_active { Some(column_idx) } else { None }
                    })
                    .collect();
            let idx = rand::thread_rng().gen_range(0..potential_columns.len());
            potential_columns[idx]
        }
        DecisionStrategy::Optimal => {
            let mut min_count = i32::MAX;
            let mut min_column_idxs = Vec::new();
            for (column_idx, is_active) in active_columns.iter().enumerate() {
                if !is_active {
                    continue;
                }

                let col = column_idx;
                if let Link::ColumnHeader(ch) = table.table[0][col] {
                    let count = ch.cell_count;
                    if count < min_count {
                        min_count = count;
                        min_column_idxs.clear();
                        min_column_idxs.push(col);
                    } else if count == min_count {
                        min_column_idxs.push(col);
                    }
                }
            }
            let idx = rand::thread_rng().gen_range(0..min_column_idxs.len());
            min_column_idxs[idx]
        }
    }
}

fn find_satisfying_rows(
    selected_column_idx: usize,
    table: &LinkedTable,
) -> Option<Vec<usize>> {
    // Literally just go to the ColumnHeader and find down
    let mut next_row = match table.table[0][selected_column_idx] {
        Link::ColumnHeader(ch) => match ch.down {
            Some(0) | None => return None,
            Some(row_idx) => row_idx,
        },
        _ => panic!(),
    };

    let mut satisfying_rows: Vec<usize> = vec![next_row];

    loop {
        next_row = match table.table[next_row][selected_column_idx] {
            Link::Cell(c) => c.down.unwrap(),
            _ => panic!(),
        };

        if next_row == 0 {
            break;
        }

        satisfying_rows.push(next_row);
    }

    Some(satisfying_rows)
}

fn pick_row(
    potential_rows: &mut Vec<usize>,
    decision_strategy: DecisionStrategy,
) -> usize {
    if potential_rows.is_empty() {
        panic!("Cannot pick row from empty array");
    }

    let selected_row_index = match decision_strategy {
        DecisionStrategy::First | DecisionStrategy::Optimal => 0,
        DecisionStrategy::Random => {
            rand::thread_rng().gen_range(0..potential_rows.len())
        }
    };

    potential_rows.swap_remove(selected_row_index)
}

fn hide_column_header(column_idx: usize, table: &mut LinkedTable) {
    let ch = match table.table[0][column_idx] {
        Link::EmptyLink => panic!("cannot hide empty link"),
        Link::Cell(_) => panic!("cannot hide cell"),
        Link::ColumnHeader(ch) => ch,
    };

    match &mut table.table[0][ch.left.unwrap()] {
        Link::ColumnHeader(c) => c.right = ch.right,
        _ => panic!("invalid"),
    };

    match &mut table.table[0][ch.right.unwrap()] {
        Link::ColumnHeader(c) => c.left = ch.left,
        _ => panic!("invalid"),
    };
}

fn reveal_column_header(column_idx: usize, table: &mut LinkedTable) {
    let ch = match table.table[0][column_idx] {
        Link::EmptyLink => panic!("cannot reveal empty link"),
        Link::Cell(_) => panic!("cannot reveal cell"),
        Link::ColumnHeader(ch) => ch,
    };

    match &mut table.table[0][ch.left.unwrap()] {
        Link::ColumnHeader(c) => c.right = Some(column_idx),
        _ => panic!("invalid"),
    };

    match &mut table.table[0][ch.right.unwrap()] {
        Link::ColumnHeader(c) => c.left = Some(column_idx),
        _ => panic!("invalid"),
    };
}

/// Hides a Link::Cell by updating the cells above and below to point around
/// the specified cell.
fn hide_cell(row_idx: usize, column_idx: usize, table: &mut LinkedTable) {
    let cell = match table.table[row_idx][column_idx] {
        Link::EmptyLink => panic!("cannot hide empty link"),
        Link::ColumnHeader(_) => panic!("cannot hide column header"),
        Link::Cell(c) => c,
    };

    match &mut table.table[cell.up.unwrap()][column_idx] {
        Link::EmptyLink => panic!("invalid"),
        Link::Cell(above_cell) => above_cell.down = cell.down,
        Link::ColumnHeader(above_ch) => above_ch.down = cell.down,
    }

    match &mut table.table[cell.down.unwrap()][column_idx] {
        Link::EmptyLink => panic!("invalid"),
        Link::Cell(below_cell) => below_cell.up = cell.up,
        Link::ColumnHeader(below_ch) => below_ch.up = cell.up,
    }

    match &mut table.table[0][column_idx] {
        Link::ColumnHeader(ch) => ch.cell_count -= 1,
        _ => panic!("invalid"),
    }
}

fn reveal_cell(row_idx: usize, column_idx: usize, table: &mut LinkedTable) {
    let cell = match table.table[row_idx][column_idx] {
        Link::EmptyLink => panic!("cannot reveal empty link"),
        Link::ColumnHeader(_) => panic!("cannot reveal column header"),
        Link::Cell(c) => c,
    };

    match &mut table.table[cell.up.unwrap()][column_idx] {
        Link::EmptyLink => panic!("invalid"),
        Link::Cell(above_cell) => above_cell.down = Some(row_idx),
        Link::ColumnHeader(above_ch) => above_ch.down = Some(row_idx),
    }

    match &mut table.table[cell.down.unwrap()][column_idx] {
        Link::EmptyLink => panic!("invalid"),
        Link::Cell(below_cell) => below_cell.up = Some(row_idx),
        Link::ColumnHeader(below_ch) => below_ch.up = Some(row_idx),
    }

    match &mut table.table[0][column_idx] {
        Link::ColumnHeader(ch) => ch.cell_count += 1,
        _ => panic!("invalid"),
    }
}

fn cover_column(selected_column_idx: usize, table: &mut LinkedTable) {
    let ch = match table.table[0][selected_column_idx] {
        Link::EmptyLink => panic!("should be column header"),
        Link::Cell(_) => panic!("should be column header"),
        Link::ColumnHeader(ch) => ch,
    };

    let mut next_row_idx = ch
        .down
        .expect("Column header should never have a none down");

    // Traverse columns with adjacent cells and hide them
    while next_row_idx != 0 {
        // Need to hide all cells in row `next_row_idx`
        let mut next_column_idx =
            match table.table[next_row_idx][selected_column_idx] {
                Link::EmptyLink => panic!("Should never point to empty link"),
                Link::ColumnHeader(ch) => ch.right.unwrap(),
                Link::Cell(c) => c.right.unwrap(),
            };

        while next_column_idx != selected_column_idx {
            // Hide this cell then update next column idx
            hide_cell(next_row_idx, next_column_idx, table);

            next_column_idx = match table.table[next_row_idx][next_column_idx] {
                Link::EmptyLink => panic!("Should never point to empty link"),
                Link::ColumnHeader(ch) => ch.right.unwrap(),
                Link::Cell(c) => c.right.unwrap(),
            };
        }

        next_row_idx = match table.table[next_row_idx][selected_column_idx] {
            Link::EmptyLink => panic!("invalid"),
            Link::ColumnHeader(ch) => ch.down.unwrap(),
            Link::Cell(c) => c.down.unwrap(),
        };
    }

    // Unlink the column header
    hide_column_header(selected_column_idx, table);
}

fn reveal_column(selected_column_idx: usize, table: &mut LinkedTable) {
    let ch = match table.table[0][selected_column_idx] {
        Link::EmptyLink => panic!("should be column header"),
        Link::Cell(_) => panic!("should be column header"),
        Link::ColumnHeader(ch) => ch,
    };

    let mut next_row_idx = ch.up.expect("Column header should have an up");

    while next_row_idx != 0 {
        let mut next_column_idx =
            match table.table[next_row_idx][selected_column_idx] {
                Link::EmptyLink => panic!("Should never point to empty link"),
                Link::ColumnHeader(ch) => ch.left.unwrap(),
                Link::Cell(c) => c.left.unwrap(),
            };

        while next_column_idx != selected_column_idx {
            reveal_cell(next_row_idx, next_column_idx, table);

            next_column_idx = match table.table[next_row_idx][next_column_idx] {
                Link::EmptyLink => panic!("Should never point to empty link"),
                Link::ColumnHeader(ch) => ch.left.unwrap(),
                Link::Cell(c) => c.left.unwrap(),
            };
        }

        next_row_idx = match table.table[next_row_idx][selected_column_idx] {
            Link::EmptyLink => panic!("invalid"),
            Link::ColumnHeader(ch) => ch.up.unwrap(),
            Link::Cell(c) => c.up.unwrap(),
        };
    }

    reveal_column_header(selected_column_idx, table);
}

fn generate_linked_table() -> LinkedTable {
    let mut table = LinkedTable::default();
    link_unlinked_table(&mut table);
    table
}

/// Hides the column at table[row_idx][column_idx], iteratively traverses the
/// right pointer hiding each column until returning back to column_idx.
fn hide_all_columns_in_row(
    row_idx: usize,
    column_idx: usize,
    table: &mut LinkedTable,
) -> Vec<usize> {
    let mut next_column_idx = column_idx;
    let mut hidden_columns = vec![];

    loop {
        hidden_columns.push(next_column_idx);
        cover_column(next_column_idx, table);
        next_column_idx = match table.table[row_idx][next_column_idx] {
            Link::Cell(c) => c.right.unwrap(),
            _ => panic!(),
        };

        if next_column_idx == column_idx {
            break;
        }
    }

    hidden_columns
}

fn reveal_all_columns_in_row(
    row_idx: usize,
    column_idx: usize,
    table: &mut LinkedTable,
) {
    let mut next_column_idx = match table.table[row_idx][column_idx] {
        Link::Cell(c) => c.left.unwrap(),
        _ => panic!(),
    };

    loop {
        reveal_column(next_column_idx, table);

        if next_column_idx == column_idx {
            break;
        }

        next_column_idx = match table.table[row_idx][next_column_idx] {
            Link::Cell(c) => c.left.unwrap(),
            _ => panic!(),
        };
    }
}

fn backtrack(
    decisions: &mut Vec<Decision>,
    active_columns: &mut [bool; LINKED_TABLE_COLUMNS],
    solution: &mut Vec<usize>,
    table: &mut LinkedTable,
    row_decision_strategy: DecisionStrategy,
) {
    loop {
        let mut previous_decision = decisions.pop().unwrap_or_else(|| {
            panic!("Dancing Links search could not find a valid solution")
        });

        reveal_all_columns_in_row(
            previous_decision.selected_row,
            previous_decision.selected_column,
            table,
        );
        for &column_idx in &previous_decision.hidden_columns {
            active_columns[column_idx] = true;
        }
        solution.pop();

        if previous_decision.potential_rows.is_empty() {
            continue;
        }

        let next_row = pick_row(
            &mut previous_decision.potential_rows,
            row_decision_strategy,
        );
        let hidden_columns = hide_all_columns_in_row(
            next_row,
            previous_decision.selected_column,
            table,
        );
        for &column_idx in &hidden_columns {
            active_columns[column_idx] = false;
        }
        solution.push(next_row);
        decisions.push(Decision {
            selected_column: previous_decision.selected_column,
            selected_row: next_row,
            potential_rows: previous_decision.potential_rows,
            hidden_columns,
        });

        return;
    }
}

fn launch_dancing_links(
    num_solutions: i32,
    column_decision_strategy: DecisionStrategy,
    row_decision_strategy: DecisionStrategy,
) -> Vec<[usize; 81]> {
    let mut solutions: Vec<[usize; 81]> =
        Vec::with_capacity(num_solutions as usize);
    let mut linked_table = generate_linked_table();
    // TODO - Figure out if this would be better as a vector
    let mut active_columns = [true; LINKED_TABLE_COLUMNS];
    let mut solution = Vec::with_capacity(81);
    let mut decisions: Vec<Decision> = Vec::with_capacity(81);

    loop {
        if active_columns.iter().all(|is_active| !is_active) {
            solutions
                .push(std::array::from_fn::<_, 81, _>(|i| solution[i] - 1));
            backtrack(
                &mut decisions,
                &mut active_columns,
                &mut solution,
                &mut linked_table,
                row_decision_strategy,
            );
        }

        if solutions.len() >= num_solutions as usize {
            break;
        }

        let selected_column = select_column(
            &active_columns,
            column_decision_strategy,
            &linked_table,
        );
        let mut candidate_rows =
            match find_satisfying_rows(selected_column, &linked_table) {
                Some(rows) => rows,
                None => {
                    backtrack(
                        &mut decisions,
                        &mut active_columns,
                        &mut solution,
                        &mut linked_table,
                        DecisionStrategy::First,
                    );
                    continue;
                }
            };

        let selected_row = pick_row(&mut candidate_rows, row_decision_strategy);
        let hidden_columns = hide_all_columns_in_row(
            selected_row,
            selected_column,
            &mut linked_table,
        );
        for &column_idx in &hidden_columns {
            active_columns[column_idx] = false;
        }
        solution.push(selected_row);
        decisions.push(Decision {
            selected_column,
            selected_row,
            potential_rows: candidate_rows,
            hidden_columns,
        });
    }

    solutions
}

pub fn advanced_get_sudoku_boards(
    num_solutions: i32,
    column_decision_strategy: DecisionStrategy,
    row_decision_strategy: DecisionStrategy,
) -> Vec<Board> {
    launch_dancing_links(
        num_solutions,
        column_decision_strategy,
        row_decision_strategy,
    )
    .into_iter()
    .map(|solution_set| map_solution_set_to_board(&HashSet::from(solution_set)))
    .collect()
}

pub fn get_sudoku_boards(num_solutions: i32) -> Vec<Board> {
    launch_dancing_links(
        num_solutions,
        DecisionStrategy::Optimal,
        DecisionStrategy::Random,
    )
    .into_iter()
    .map(|solution_set| map_solution_set_to_board(&HashSet::from(solution_set)))
    .collect()
}
