use rand::Rng;
use std::array;

use crate::{
    Board,
    algorithm_x::{ConstraintTable, generate_constraint_table},
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
enum DecisionStrategy {
    First,
    Random,
    Optimal,
}

impl Default for LinkedTable {
    fn default() -> Self {
        let mut vec_table = vec![];

        for _index in 0..LINKED_TABLE_ROWS {
            vec_table.push([Link::EmptyLink; LINKED_TABLE_COLUMNS]);
        }

        let typed_boxed_table: Box<[[Link; LINKED_TABLE_COLUMNS]; LINKED_TABLE_ROWS]> =
            match vec_table.into_boxed_slice().try_into() {
                Ok(result) => result,
                Err(_err) => panic!("unable to initialize empty dancing link table"),
            };

        let constraint_table = generate_constraint_table();
        let mut linked_table = Self {
            table: typed_boxed_table,
        };

        linked_table.table[0] = generate_column_headers(&constraint_table);
        linked_table.table[1..].clone_from_slice(&*generate_unlinked_rows(&constraint_table));

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

fn generate_unlinked_rows(constraint_table: &ConstraintTable) -> Box<[[Link; 324]; 729]> {
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
                Link::ColumnHeader(column_header) => column_header.right = Some(column_index),
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
            Link::ColumnHeader(column_header) => column_header.left = Some(last_link_index),
            Link::Cell(cell) => cell.left = Some(last_link_index),
        };

        match &mut table[row_index][last_link_index] {
            Link::EmptyLink => (),
            Link::ColumnHeader(column_header) => column_header.right = Some(first_link_index),
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
                Link::ColumnHeader(column_header) => column_header.up = last_link_index,
                Link::Cell(cell) => cell.up = last_link_index,
            };

            match &mut table[last_link_index.unwrap()][column_index] {
                Link::EmptyLink => (),
                Link::ColumnHeader(column_header) => column_header.down = Some(row_index),
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
            Link::ColumnHeader(column_header) => column_header.up = Some(last_link_index),
            Link::Cell(cell) => cell.up = Some(last_link_index),
        };

        match &mut table[last_link_index][column_index] {
            Link::EmptyLink => (),
            Link::ColumnHeader(column_header) => column_header.down = Some(first_link_index),
            Link::Cell(cell) => cell.down = Some(first_link_index),
        };
    }
}

/// Selects a column from the list of potential columns according to the strategy.
/// Returns (selected_column, remaining_columns)
fn select_column(
    mut potential_columns: Vec<usize>,
    decision_strategy: DecisionStrategy,
    table: &LinkedTable,
) -> (usize, Vec<usize>) {
    if potential_columns.is_empty() {
        panic!("No columns to select");
    }
    match decision_strategy {
        DecisionStrategy::First => {
            let selected = potential_columns.pop().unwrap();
            (selected, potential_columns)
        }
        DecisionStrategy::Random => {
            let idx = rand::thread_rng().gen_range(0..potential_columns.len());
            let selected = potential_columns.remove(idx);
            (selected, potential_columns)
        }
        DecisionStrategy::Optimal => {
            let mut rng = rand::thread_rng();
            let mut min_count = i32::MAX;
            let mut min_positions = Vec::new();
            for (i, &col) in potential_columns.iter().enumerate() {
                if let Link::ColumnHeader(ch) = table.table[0][col] {
                    let count = ch.cell_count;
                    if count < min_count {
                        min_count = count;
                        min_positions.clear();
                        min_positions.push(i);
                    } else if count == min_count {
                        min_positions.push(i);
                    }
                }
            }
            if min_positions.is_empty() {
                panic!("No columns to select");
            }
            let pick = rng.gen_range(0..min_positions.len());
            let pos = min_positions[pick];
            let selected = potential_columns.remove(pos);
            (selected, potential_columns)
        }
    }
}

fn find_satisfying_row(selected_column_idx: usize, table: &LinkedTable) -> usize {
    // Literally just go to the ColumnHeader and find down
    match table.table[0][selected_column_idx] {
        Link::EmptyLink => panic!("must be a column header"),
        Link::Cell(_) => panic!("must be a column header"),
        Link::ColumnHeader(ch) => {
            if ch.down == None || ch.down == Some(0) {
                panic!(
                    "There are no rows that satisfy the constraint at column: {}",
                    selected_column_idx
                );
            }

            return ch.down.unwrap();
        }
    };
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
        let mut next_column_idx = match table.table[next_row_idx][selected_column_idx] {
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

fn generate_linked_table() -> LinkedTable {
    let mut table = LinkedTable::default();
    link_unlinked_table(&mut table);
    table
}

pub fn launch_dancing_links() -> Vec<Board> {
    let mut linked_table = generate_linked_table();

    let mut unsatisfied_columns: Vec<usize> = (0..LINKED_TABLE_COLUMNS).collect();
    let mut solution_set: Vec<usize> = vec![];

    loop {
        let (selected_column_idx, new_unsatisfied_columns) =
            select_column(unsatisfied_columns, DecisionStrategy::First, &linked_table);
        unsatisfied_columns = new_unsatisfied_columns;

        let selected_row_idx = find_satisfying_row(selected_column_idx, &linked_table);
        solution_set.push(selected_row_idx);

        let mut current_column_idx = selected_column_idx;
        loop {
            cover_column(current_column_idx, &mut linked_table);

            current_column_idx = match linked_table.table[selected_row_idx][current_column_idx] {
                Link::Cell(c) => c.right.unwrap(),
                _ => panic!("must be a cell"),
            };

            if current_column_idx == selected_column_idx {
                break;
            }
        }

        if true {
            break;
        }
    }

    vec![]
}
