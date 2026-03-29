use std::ptr;

use super::*;
use crate::algorithm_x::generate_constraint_table;

#[test]
fn it_can_construct_an_empty_table() {
    let mut left = true;
    let mut column_index = 323;

    let dancing_table = LinkedTable::default();

    for row_index in 0..LINKED_TABLE_ROWS {
        match dancing_table.table[row_index][column_index] {
            Link::EmptyLink => (),
            Link::ColumnHeader(ch) => assert_eq!(
                ch,
                ColumnHeader {
                    cell_count: 9,
                    up: None,
                    down: None,
                    left: None,
                    right: None,
                }
            ),
            Link::Cell(cell) => assert_eq!(
                cell,
                Cell {
                    column_index: column_index,
                    row_index: row_index - 1,
                    up: None,
                    down: None,
                    left: None,
                    right: None,
                }
            ),
        };

        if column_index == 0 {
            left = false;
        }
        if column_index == 323 {
            left = true;
        }

        if left {
            column_index -= 1;
        } else {
            column_index += 1;
        }
    }
}

#[test]
fn test_generate_column_headers() {
    let constraint_table = generate_constraint_table();
    let headers = generate_column_headers(&constraint_table);
    let correct_header = Link::ColumnHeader(ColumnHeader {
        cell_count: 9,
        up: None,
        down: None,
        left: None,
        right: None,
    });

    for header in headers {
        assert_eq!(header, correct_header);
    }
}

#[test]
fn it_generates_unlinked_rows() {
    let constraint_table = generate_constraint_table();
    let rows = generate_unlinked_rows(&constraint_table);

    let num_cells_first_row = rows[0].iter().filter(|x| **x != Link::EmptyLink).count();

    assert_eq!(num_cells_first_row, 4);

    let mut num_cells_last_column = 0;
    for index in 0..729 {
        if rows[index][323] != Link::EmptyLink {
            num_cells_last_column += 1;
        }
    }

    assert_eq!(num_cells_last_column, 9);
}

#[test]
fn it_generates_a_linked_table() {
    let unlinked_table = generate_linked_table();
    assert_eq!(unlinked_table.table.len(), LINKED_TABLE_ROWS);
}

#[test]
fn it_links_the_rows_in_an_uninitialized_table() {
    // Each row has 4 constraint categories
    // Each choice will fill one constraint category
    // --> Each row should be a circularly linked list with 4 elements
    let mut linked_table = LinkedTable::default();
    link_unlinked_table(&mut linked_table);

    // Verify that all non-edge rows are linked with eachother
    for index in 1..LINKED_TABLE_COLUMNS - 1 {
        let ch_left_index = match &linked_table.table[0][index] {
            Link::EmptyLink => None,
            Link::ColumnHeader(ch) => ch.left,
            Link::Cell(_) => None,
        };

        let ch_right_index = match &linked_table.table[0][index] {
            Link::EmptyLink => None,
            Link::ColumnHeader(ch) => ch.right,
            Link::Cell(_) => None,
        };

        assert_eq!(ch_left_index, Some(index - 1));
        assert_eq!(ch_right_index, Some(index + 1));
    }

    // Verify the edge columns are circularly linked
    let first_index_right = match &linked_table.table[0][0] {
        Link::EmptyLink => None,
        Link::Cell(_) => None,
        Link::ColumnHeader(ch) => ch.right,
    };

    let first_index_left = match &linked_table.table[0][0] {
        Link::EmptyLink => None,
        Link::Cell(_) => None,
        Link::ColumnHeader(ch) => ch.left,
    };

    assert_eq!(first_index_right, Some(1));
    assert_eq!(first_index_left, Some(LINKED_TABLE_COLUMNS - 1));

    let last_index_right = match &linked_table.table[0][LINKED_TABLE_COLUMNS - 1] {
        Link::EmptyLink => None,
        Link::Cell(_) => None,
        Link::ColumnHeader(ch) => ch.right,
    };

    let last_index_left = match &linked_table.table[0][LINKED_TABLE_COLUMNS - 1] {
        Link::EmptyLink => None,
        Link::Cell(_) => None,
        Link::ColumnHeader(ch) => ch.left,
    };

    assert_eq!(last_index_right, Some(0));
    assert_eq!(last_index_left, Some(LINKED_TABLE_COLUMNS - 2));

    // Here we using a trailing pointer test to make sure that after 5 iterations we have looped back and pointed at the starting cell
    for row_index in 1..LINKED_TABLE_ROWS {
        let first_element = linked_table.table[row_index]
            .iter()
            .find(|e| **e != Link::EmptyLink);
        assert!(first_element.is_some());
        let first_element = first_element.unwrap();

        let mut fifth_element = first_element;
        for _iteration in 0..4 {
            let next_index = match fifth_element {
                Link::EmptyLink => panic!("Invalid state"),
                Link::ColumnHeader(col_head) => col_head.right.unwrap(),
                Link::Cell(cell) => cell.right.unwrap(),
            };
            fifth_element = &linked_table.table[row_index][next_index];
        }
        assert!(ptr::eq(first_element, fifth_element));
    }
}

#[test]
fn it_links_the_columns_in_an_uninitlized_table() {
    // Every column should have 9 cells + the column header
    let mut linked_table = generate_linked_table();
    link_unlinked_table(&mut linked_table);

    for column_index in 0..LINKED_TABLE_COLUMNS {
        let mut first_cell: Option<&Link> = None;
        for row_index in 0..LINKED_TABLE_ROWS {
            match linked_table.table[row_index][column_index] {
                Link::EmptyLink => (),
                Link::ColumnHeader(_) => (),
                Link::Cell(_) => {
                    first_cell = Some(&linked_table.table[row_index][column_index]);
                    break;
                }
            };
        }
        let first_cell = first_cell.unwrap();

        let mut tenth_link = first_cell;
        for _index in 0..10 {
            let _i = 0;
            match *tenth_link {
                Link::EmptyLink => assert!(false),
                Link::ColumnHeader(ch) => {
                    let next_index = ch.down.unwrap();
                    tenth_link = &linked_table.table[next_index][column_index];
                }
                Link::Cell(cell) => {
                    let next_index = cell.down.unwrap();
                    tenth_link = &linked_table.table[next_index][column_index];
                }
            }
        }
        assert!(ptr::eq(first_cell, tenth_link));
    }
}
