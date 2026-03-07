use std::ptr;

use super::*;
use crate::algorithm_x::generate_constraint_table;

#[test]
fn it_can_construct_an_empty_table() {
    let mut left = true;
    let mut column_index = 323;

    let dancing_table = LinkedTable::default();

    for row_index in 0..LINKED_TABLE_ROWS {
        assert_eq!(
            dancing_table.table[row_index][column_index],
            Link::EmptyLink
        );
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
fn it_links_the_columns_in_an_uninitialized_table() {
    // Each row has 4 constraint categories
    // Each choice will fill one constraint category
    // --> Each row should be a circularly linked list with 4 elements
    let constraint_table = generate_constraint_table();
    let mut linked_table = LinkedTable::default();
    linked_table.table[0] = generate_column_headers(&constraint_table);
    linked_table.table[1..].clone_from_slice(&*generate_unlinked_rows(&constraint_table));
    link_linked_table(&mut linked_table);

    // Verify that all non-edge columns are linked with eachother
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

// TODO: Link the rows together
fn it_links_the_rows_in_an_uninitlized_table() {
    todo!()
}
