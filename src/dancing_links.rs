use std::{array, collections, mem::MaybeUninit};

use crate::algorithm_x::ConstraintTable;

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

        Self {
            table: typed_boxed_table,
        }
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

fn link_linked_table(linked_table: &mut LinkedTable) -> () {
    // This for loop will link each row one after another

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
}

fn generate_linked_table() -> LinkedTable {
    LinkedTable::default()
}

pub fn launch_dancing_links() {
    ()
}
