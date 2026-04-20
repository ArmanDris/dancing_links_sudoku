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
fn it_selects_first_column_simple() {
    let lt = LinkedTable::default();
    let mut cols = vec![4, 5, 6];
    let selected = select_column(&mut cols, DecisionStrategy::First, &lt);
    assert_eq!(selected, 6);
    assert_eq!(cols, vec![4, 5]);
}

#[test]
fn it_selects_random_column_simple() {
    let lt = LinkedTable::default();
    let mut cols = vec![10, 20, 30];
    let before = cols.clone();
    let selected = select_column(&mut cols, DecisionStrategy::Random, &lt);
    assert_eq!(cols.len(), before.len() - 1);
    assert!(before.contains(&selected));
    assert!(!cols.contains(&selected));
}

#[test]
fn it_selects_optimal_column() {
    let mut linked_table = LinkedTable::default();
    link_unlinked_table(&mut linked_table);

    if let Link::ColumnHeader(ch) = &mut linked_table.table[0][5] {
        ch.cell_count = 8;
    }
    if let Link::ColumnHeader(ch) = &mut linked_table.table[0][8] {
        ch.cell_count = 8;
    }
    let mut cols = vec![0, 5, 6, 8];
    let selected = select_column(&mut cols, DecisionStrategy::Optimal, &linked_table);
    assert!(selected == 5 || selected == 8);
    assert!(cols == vec![0, 6, 8] || cols == vec![0, 5, 6]);
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

#[test]
fn it_hides_column_headers() {
    let mut linked_table = LinkedTable::default();
    link_unlinked_table(&mut linked_table);

    let original_right = match linked_table.table[0][18] {
        Link::ColumnHeader(ch) => ch.right,
        _ => panic!("invalid"),
    };

    let original_left = match linked_table.table[0][20] {
        Link::EmptyLink => panic!("invalid"),
        Link::Cell(_) => panic!("invalid"),
        Link::ColumnHeader(ch) => ch.left,
    };

    assert_eq!(original_right, Some(19));
    assert_eq!(original_left, Some(19));

    hide_column_header(19, &mut linked_table);
    hide_column_header(19, &mut linked_table);

    let new_right = match linked_table.table[0][18] {
        Link::EmptyLink => panic!("invalid"),
        Link::Cell(_) => panic!("invalid"),
        Link::ColumnHeader(ch) => ch.right,
    };

    let new_left = match linked_table.table[0][20] {
        Link::EmptyLink => panic!("invalid"),
        Link::Cell(_) => panic!("invalid"),
        Link::ColumnHeader(ch) => ch.left,
    };

    assert_eq!(new_right, Some(20));
    assert_eq!(new_left, Some(18));
}

#[test]
fn it_hides_a_cell() {
    // Original linked table column 50 looks like this:
    // ... | 0   (Down 411)  (Up 483) | ...
    // ... | ...                     | ...
    // ... | 411 (Down 0)   (Up 420) | ...
    // ... | ...                     | ...
    // ... | 420 (Down 429) (Up 411) | ...

    // After popping row 411, index 50 it shoud look like this:
    // ... | 0   (Down 420) (Up 483) | ...
    // ... | ...                     | ...
    // ... | 411 (Down 0)   (Up 420) | ...
    // ... | ...                     | ...
    // ... | 420 (Down 0)   (Up 411) | ...
    let mut linked_table = LinkedTable::default();
    link_unlinked_table(&mut linked_table);

    let ch = match linked_table.table[0][50] {
        Link::EmptyLink => panic!(),
        Link::Cell(_) => panic!(),
        Link::ColumnHeader(ch) => ch,
    };

    let c_two = match linked_table.table[411][50] {
        Link::EmptyLink => panic!(),
        Link::ColumnHeader(_) => panic!(),
        Link::Cell(c) => c,
    };

    let c_three = match linked_table.table[420][50] {
        Link::EmptyLink => panic!(),
        Link::ColumnHeader(_) => panic!(),
        Link::Cell(c) => c,
    };

    assert_eq!(ch.down, Some(411));
    assert_eq!(c_two.up, Some(0));
    assert_eq!(c_two.down, Some(420));
    assert_eq!(c_three.up, Some(411));

    hide_cell(411, 50, &mut linked_table);

    let ch_after = match linked_table.table[0][50] {
        Link::EmptyLink => panic!(),
        Link::Cell(_) => panic!(),
        Link::ColumnHeader(ch) => ch,
    };

    let c_two_after = match linked_table.table[411][50] {
        Link::EmptyLink => panic!(),
        Link::ColumnHeader(_) => panic!(),
        Link::Cell(c) => c,
    };

    let c_three_after = match linked_table.table[420][50] {
        Link::EmptyLink => panic!(),
        Link::ColumnHeader(_) => panic!(),
        Link::Cell(c) => c,
    };
    assert_eq!(ch_after.down, Some(420));
    assert_eq!(c_two_after.up, Some(0));
    assert_eq!(c_two_after.down, Some(420));
    assert_eq!(c_three_after.up, Some(0));
    assert_eq!(ch_after.cell_count + 1, ch.cell_count);
}

#[test]
fn it_covers_a_column() {
    // When given this as input, and instruction to hide column 0
    //  ||        ||        ||
    //  v|        v|        v|
    // ______    ______    ______
    // | ch | -> | ch | -> | ch |
    // | 0  | <- | 1  | <- | 2  |
    // |cc:2|    |cc:2|    |cc:2|
    // ------    ------    ------
    //
    //  |^        |^        |^
    //  v|        v|        v|
    //
    // ______    ______    ______
    // | c  | -> | c  | -> | c  |
    // | 0  | <- | 1  | <- | 2  |
    // ------    ------    ------
    //
    //  |^        |^        |^
    //  v|        ||        ||
    //
    // ______    ______    ______
    // | c  | -> |    | -> |    |
    // | 0  | <- |    | <- |    |
    // ------    ------    ------
    //
    //  |^        ||        ||
    //  ||        v|        v|
    //  ||
    // _||___    ______    ______
    // |    | -> | c  | -> | c  |
    // |    | <- | 1  | <- | 2  |
    // -||---    ------    ------
    //  ||        |^        |^
    //  ||        ||        ||

    // It should output:
    //  ||        ||        ||
    //  v|        v|  /----------
    // ______    ______    ______ \
    // | ch |    | ch | -> | ch | /
    // | 0  |  / | 1  | <- | 2  |
    // |cc:2|    |cc:1|    |cc:1|
    // ------  | ------    ------
    //          -----------/
    //  |^        |^        |^
    //  v|        ||        ||
    //            ||        ||
    // ______    _||___    _||___
    // | c  | -> | c  | -> | c  |
    // | 0  | <- | 1  | <- | 2  |
    // ------    -||---    -||---
    //            ||        ||
    //  |^        ||        ||
    //  v|        ||        ||
    //            ||        ||
    // ______    _||___    _||___
    // | c  | -> |    | -> |    |
    // | 0  | <- |    | <- |    |
    // ------    -||---    -||---
    //            ||        ||
    //  |^        ||        ||
    //  ||        v|        v|
    //  ||
    // _||___    ______    ______
    // |    | -> | c  | -> | c  |
    // |    | <- | 1  | <- | 2  |
    // -||----    ------    ------
    //  ||        |^        |^
    //  ||        ||        ||
    //

    let mut linked_table = LinkedTable::default();
    // Here im just gonna manually hook this table up for the test
    linked_table.table[0][0] = Link::ColumnHeader(ColumnHeader {
        cell_count: 2,
        up: Some(2),
        down: Some(1),
        right: Some(1),
        left: Some(2),
    });

    linked_table.table[0][1] = Link::ColumnHeader(ColumnHeader {
        cell_count: 2,
        right: Some(2),
        left: Some(0),
        up: Some(3),
        down: Some(1),
    });

    linked_table.table[0][2] = Link::ColumnHeader(ColumnHeader {
        cell_count: 2,
        right: Some(0),
        left: Some(1),
        up: Some(3),
        down: Some(1),
    });
    // Second row
    linked_table.table[1][0] = Link::Cell(Cell {
        row_index: 1,
        column_index: 0,
        right: Some(1),
        left: Some(2),
        up: Some(0),
        down: Some(2),
    });
    linked_table.table[1][1] = Link::Cell(Cell {
        row_index: 1,
        column_index: 1,
        right: Some(2),
        left: Some(0),
        up: Some(0),
        down: Some(3),
    });
    linked_table.table[1][2] = Link::Cell(Cell {
        row_index: 1,
        column_index: 2,
        right: Some(0),
        left: Some(1),
        up: Some(0),
        down: Some(3),
    });
    // Third row
    linked_table.table[2][0] = Link::Cell(Cell {
        row_index: 2,
        column_index: 0,
        right: Some(0),
        left: Some(0),
        up: Some(1),
        down: Some(0),
    });
    // Fourth row
    linked_table.table[3][1] = Link::Cell(Cell {
        row_index: 3,
        column_index: 1,
        left: Some(2),
        right: Some(2),
        up: Some(1),
        down: Some(0),
    });
    linked_table.table[3][2] = Link::Cell(Cell {
        row_index: 3,
        column_index: 2,
        left: Some(1),
        right: Some(1),
        up: Some(1),
        down: Some(0),
    });

    // TODO! Make the initial assetions then act and make the updated
    // assertions
    assert_eq!(
        linked_table.table[0][0],
        Link::ColumnHeader(ColumnHeader {
            cell_count: 2,
            left: Some(2),
            right: Some(1),
            up: Some(2),
            down: Some(1)
        })
    );
    assert_eq!(
        linked_table.table[0][1],
        Link::ColumnHeader(ColumnHeader {
            cell_count: 2,
            up: Some(3),
            down: Some(1),
            left: Some(0),
            right: Some(2)
        })
    );
    assert_eq!(
        linked_table.table[0][2],
        Link::ColumnHeader(ColumnHeader {
            cell_count: 2,
            up: Some(3),
            down: Some(1),
            left: Some(1),
            right: Some(0)
        })
    );
    assert_eq!(
        linked_table.table[3][1],
        Link::Cell(Cell {
            column_index: 1,
            row_index: 3,
            up: Some(1),
            down: Some(0),
            left: Some(2),
            right: Some(2)
        })
    );
    assert_eq!(
        linked_table.table[3][2],
        Link::Cell(Cell {
            column_index: 2,
            row_index: 3,
            up: Some(1),
            down: Some(0),
            left: Some(1),
            right: Some(1),
        })
    );
    cover_column(0, &mut linked_table);
    assert_eq!(
        linked_table.table[0][0],
        Link::ColumnHeader(ColumnHeader {
            cell_count: 2,
            up: Some(2),
            down: Some(1),
            left: Some(2),
            right: Some(1)
        })
    );
    assert_eq!(
        linked_table.table[0][1],
        Link::ColumnHeader(ColumnHeader {
            cell_count: 1,
            up: Some(3),
            down: Some(3),
            left: Some(2),
            right: Some(2)
        })
    );
    assert_eq!(
        linked_table.table[0][2],
        Link::ColumnHeader(ColumnHeader {
            cell_count: 1,
            up: Some(3),
            down: Some(3),
            left: Some(1),
            right: Some(1)
        })
    );
    assert_eq!(
        linked_table.table[3][1],
        Link::Cell(Cell {
            column_index: 1,
            row_index: 3,
            up: Some(0),
            down: Some(0),
            left: Some(2),
            right: Some(2)
        })
    );
    assert_eq!(
        linked_table.table[3][2],
        Link::Cell(Cell {
            column_index: 2,
            row_index: 3,
            up: Some(0),
            down: Some(0),
            left: Some(1),
            right: Some(1)
        })
    );
    // Make sure the cells in the hidden column (0) are unchanged
    assert_eq!(
        linked_table.table[1][0],
        Link::Cell(Cell {
            column_index: 0,
            row_index: 1,
            up: Some(0),
            down: Some(2),
            left: Some(2),
            right: Some(1),
        })
    );
    assert_eq!(
        linked_table.table[2][0],
        Link::Cell(Cell {
            column_index: 0,
            row_index: 2,
            up: Some(1),
            down: Some(0),
            left: Some(0),
            right: Some(0),
        })
    );
}

#[test]
fn it_finds_all_expected_rows() {
    let mut linked_table = LinkedTable::default();
    link_unlinked_table(&mut linked_table);

    let (selected_row, alternate_rows) = find_satisfying_rows(14, &mut linked_table);

    assert_eq!(selected_row, 87);
    assert_eq!(alternate_rows, vec![96, 105, 114, 123, 132, 141, 150, 159]);
}
