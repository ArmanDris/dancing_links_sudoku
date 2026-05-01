use std::ptr;
use std::{collections::HashSet, fs};

use super::*;
use crate::algorithm_x::generate_constraint_table;

fn assert_board_is_solved_and_valid(board: &Board) {
    for row_idx in 0..9 {
        let row = board.get_row(row_idx);
        let mut sorted_row = row.to_vec();
        sorted_row.sort_unstable();
        assert_eq!(sorted_row, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    for column_idx in 0..9 {
        let mut column = board.get_column(column_idx).to_vec();
        column.sort_unstable();
        assert_eq!(column, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    for subgrid_y in (0..9).step_by(3) {
        for subgrid_x in (0..9).step_by(3) {
            let mut subgrid = Vec::with_capacity(9);
            for y in subgrid_y..subgrid_y + 3 {
                for x in subgrid_x..subgrid_x + 3 {
                    subgrid.push(board.get(x, y));
                }
            }
            subgrid.sort_unstable();
            assert_eq!(subgrid, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        }
    }
}

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
    let mut cols = [false; LINKED_TABLE_COLUMNS];
    cols[4] = true;
    cols[5] = true;
    cols[6] = true;
    let selected = select_column(&cols, DecisionStrategy::First, &lt);
    assert_eq!(selected, 4);
}

#[test]
fn it_selects_random_column_simple() {
    let lt = LinkedTable::default();
    let mut cols = [false; LINKED_TABLE_COLUMNS];
    cols[10] = true;
    cols[20] = true;
    cols[30] = true;
    let selected = select_column(&cols, DecisionStrategy::Random, &lt);
    assert!(selected == 10 || selected == 20 || selected == 30);
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
    let mut cols = [false; LINKED_TABLE_COLUMNS];
    cols[0] = true;
    cols[5] = true;
    cols[6] = true;
    cols[8] = true;
    let selected =
        select_column(&cols, DecisionStrategy::Optimal, &linked_table);
    assert!(selected == 5 || selected == 8);
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

    let num_cells_first_row =
        rows[0].iter().filter(|x| **x != Link::EmptyLink).count();

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

    let last_index_right =
        match &linked_table.table[0][LINKED_TABLE_COLUMNS - 1] {
            Link::EmptyLink => None,
            Link::Cell(_) => None,
            Link::ColumnHeader(ch) => ch.right,
        };

    let last_index_left = match &linked_table.table[0][LINKED_TABLE_COLUMNS - 1]
    {
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
                    first_cell =
                        Some(&linked_table.table[row_index][column_index]);
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

    let satisfying_rows = find_satisfying_rows(14, &linked_table);

    assert_eq!(
        satisfying_rows,
        Some(vec![87, 96, 105, 114, 123, 132, 141, 150, 159])
    );
}

#[test]
fn it_returns_none_when_column_has_no_cells() {
    let mut linked_table = LinkedTable::default();

    linked_table.table[0][0] = Link::ColumnHeader(ColumnHeader {
        cell_count: 0,
        up: None,
        down: None,
        left: Some(0),
        right: Some(0),
    });

    assert_eq!(find_satisfying_rows(0, &linked_table), None);
}

fn generate_crafted_hide_all_columns_in_row_table() -> LinkedTable {
    let typed_boxed_table: Box<
        [[Link; LINKED_TABLE_COLUMNS]; LINKED_TABLE_ROWS],
    > = vec![[Link::EmptyLink; LINKED_TABLE_COLUMNS]; LINKED_TABLE_ROWS]
        .into_boxed_slice()
        .try_into()
        .unwrap();

    let mut linked_table = LinkedTable {
        table: typed_boxed_table,
    };
    // Column headers
    linked_table.table[0][0] = Link::ColumnHeader(ColumnHeader {
        cell_count: 2,
        up: Some(4),
        down: Some(2),
        left: Some(6),
        right: Some(1),
    });
    linked_table.table[0][1] = Link::ColumnHeader(ColumnHeader {
        cell_count: 2,
        up: Some(5),
        down: Some(3),
        left: Some(0),
        right: Some(2),
    });
    linked_table.table[0][2] = Link::ColumnHeader(ColumnHeader {
        cell_count: 2,
        up: Some(3),
        down: Some(1),
        left: Some(1),
        right: Some(3),
    });
    linked_table.table[0][3] = Link::ColumnHeader(ColumnHeader {
        cell_count: 3,
        up: Some(6),
        down: Some(2),
        left: Some(2),
        right: Some(4),
    });
    linked_table.table[0][4] = Link::ColumnHeader(ColumnHeader {
        cell_count: 2,
        up: Some(6),
        down: Some(1),
        left: Some(3),
        right: Some(5),
    });
    linked_table.table[0][5] = Link::ColumnHeader(ColumnHeader {
        cell_count: 2,
        up: Some(3),
        down: Some(1),
        left: Some(4),
        right: Some(6),
    });
    linked_table.table[0][6] = Link::ColumnHeader(ColumnHeader {
        cell_count: 3,
        up: Some(6),
        down: Some(2),
        left: Some(5),
        right: Some(0),
    });
    // Row 1
    linked_table.table[1][2] = Link::Cell(Cell {
        column_index: 2,
        row_index: 1,
        up: Some(0),
        down: Some(3),
        left: Some(5),
        right: Some(4),
    });
    linked_table.table[1][4] = Link::Cell(Cell {
        column_index: 4,
        row_index: 1,
        up: Some(0),
        down: Some(6),
        left: Some(2),
        right: Some(5),
    });
    linked_table.table[1][5] = Link::Cell(Cell {
        column_index: 5,
        row_index: 1,
        up: Some(0),
        down: Some(3),
        left: Some(4),
        right: Some(2),
    });
    // Row 2
    linked_table.table[2][0] = Link::Cell(Cell {
        column_index: 0,
        row_index: 2,
        up: Some(0),
        down: Some(4),
        left: Some(6),
        right: Some(3),
    });
    linked_table.table[2][3] = Link::Cell(Cell {
        column_index: 3,
        row_index: 2,
        up: Some(0),
        down: Some(4),
        left: Some(0),
        right: Some(6),
    });
    linked_table.table[2][6] = Link::Cell(Cell {
        column_index: 6,
        row_index: 2,
        up: Some(0),
        down: Some(5),
        left: Some(3),
        right: Some(0),
    });
    // Row 3
    linked_table.table[3][1] = Link::Cell(Cell {
        column_index: 1,
        row_index: 3,
        up: Some(0),
        down: Some(5),
        left: Some(5),
        right: Some(2),
    });
    linked_table.table[3][2] = Link::Cell(Cell {
        column_index: 2,
        row_index: 3,
        up: Some(1),
        down: Some(0),
        left: Some(1),
        right: Some(5),
    });
    linked_table.table[3][5] = Link::Cell(Cell {
        column_index: 5,
        row_index: 3,
        up: Some(1),
        down: Some(0),
        left: Some(2),
        right: Some(1),
    });
    // Row 4
    linked_table.table[4][0] = Link::Cell(Cell {
        column_index: 0,
        row_index: 4,
        up: Some(2),
        down: Some(0),
        left: Some(3),
        right: Some(3),
    });
    linked_table.table[4][3] = Link::Cell(Cell {
        column_index: 3,
        row_index: 4,
        up: Some(2),
        down: Some(6),
        left: Some(0),
        right: Some(0),
    });
    // Row 5
    linked_table.table[5][1] = Link::Cell(Cell {
        column_index: 1,
        row_index: 5,
        up: Some(3),
        down: Some(0),
        left: Some(6),
        right: Some(6),
    });
    linked_table.table[5][6] = Link::Cell(Cell {
        column_index: 6,
        row_index: 5,
        up: Some(2),
        down: Some(6),
        left: Some(1),
        right: Some(1),
    });
    // Row 6
    linked_table.table[6][3] = Link::Cell(Cell {
        column_index: 3,
        row_index: 6,
        up: Some(4),
        down: Some(0),
        left: Some(6),
        right: Some(4),
    });
    linked_table.table[6][4] = Link::Cell(Cell {
        column_index: 4,
        row_index: 6,
        up: Some(1),
        down: Some(0),
        left: Some(3),
        right: Some(6),
    });
    linked_table.table[6][6] = Link::Cell(Cell {
        column_index: 6,
        row_index: 6,
        up: Some(5),
        down: Some(0),
        left: Some(4),
        right: Some(3),
    });

    linked_table
}

fn assert_crafted_table_eq(actual: &LinkedTable, expected: &LinkedTable) {
    for row_idx in 0..7 {
        for column_idx in 0..7 {
            assert_eq!(
                actual.table[row_idx][column_idx],
                expected.table[row_idx][column_idx],
                "mismatch at row {row_idx}, column {column_idx}"
            );
        }
    }
}

#[test]
fn it_coveres_all_columns_for_a_row() {
    // Starting with this grid:
    //
    //      | |      | |      | |      | |      | |      | |      | |
    //      | v      | v      | V      | V      | V      | v      | v
    // --->  A  --->  B  --->  C  --->  D  --->  E  --->  F  --->  G  ---
    // ---- (2) <--- (2) <--- (2) <--- (3) <--- (2) <--- (2) <--- (3) <--
    //      ^ |      ^ |      ^ |      ^ |      ^ |      ^ |      ^ |
    //      | |      | |      | |      | |      | |      | |      | |
    //      | |      | |      | V      | |      | V      | v      | |
    //      | |      | |     [   ]     | |     [   ]    [   ]     | |
    //      | |      | |     [   ]     | |     [   ]    [   ]     | |
    //      | |      | |      ^ |      | |      ^ |      ^ |      | |
    //      | |      | |      | |      | |      | |      | |      | |
    //      | v      | |      | |      | V      | |      | |      | v
    //     [   ]     | |      | |     [   ]     | |      | |     [   ]
    //     [   ]     | |      | |     [   ]     | |      | |     [   ]
    //      ^ |      | |      | |      ^ |      | |      | |      ^ |
    //      | |      | |      | |      | |      | |      | |      | |
    //      | |      | v      | V      | |      | |      | V      | |
    //      | |     [   ]    [   ]     | |      | |     [   ]     | |
    //      | |     [   ]    [   ]     | |      | |     [   ]     | |
    //      | |      ^ |      ^ |      | |      | |      ^ |      | |
    //      | |      | |      | |      | |      | |      | |      | |
    //      | v      | |      | |      | V      | |      | |      | |
    //     [   ]     | |      | |     [   ]     | |      | |      | |
    //     [   ]     | |      | |     [   ]     | |      | |      | |
    //      ^ |      | |      | |      ^ |      | |      | |      | |
    //      | |      | |      | |      | |      | |      | |      | |
    //      | |      | v      | |      | |      | |      | |      | v
    //      | |     [   ]     | |      | |      | |      | |     [   ]
    //      | |     [   ]     | |      | |      | |      | |     [   ]
    //      | |      ^ |      | |      | |      | |      | |      ^ |
    //      | |      | |      | |      | |      | |      | |      | |
    //      | |      | |      | |      | V      | v      | |      | v
    //      | |      | |      | |     [   ]    [   ]     | |     [   ]
    //      | |      | |      | |     [   ]    [   ]     | |     [   ]
    //      | |      | |      | |      ^ |      ^ |      | |      ^ |
    //      | |      | |      | |      | |      | |      | |      | |
    //
    // If we cover column A, then D, then G then we should have:
    //      | |      | |      | |      | |      | |      | |      | |
    //     -|-v-     | v      | V     -|-V-     | V      | v     -|-v-
    // ---/  A -\-->  B  --->  C  ---/  D -\-->  E  --->  F  ---/  G -\--
    // ---\-(1) /--- (1) <--- (2) <--\-(1) /--- (1) <--- (2) <--\-(1) /--
    //     -^-|-     ^ |      ^ |     \^-|-     ^ |      ^ |     -^-|-
    //      | |      | |      | |      | |      | |      | |      | |
    //      | |      | |      | V      | |      | V      | v      | |
    //      | |      | |     [   ]     | |     [   ]    [   ]     | |
    //      | |      | |     [   ]     | |     [   ]    [   ]     | |
    //      | |      | |      ^ |      | |      ^ |      ^ |      | |
    //      | |      | |      | |     /| \      | |      | |     /| \
    //      | v      | |      | |    / |   \    | |      | |    / |   \
    //     [   ]     | |      | |   | [   ] |   | |      | |   | [   ] |
    //     [   ]     | |      | |   | [   ] |   | |      | |   | [   ] |
    //      ^ |      | |      | |    \   | /    | |      | |    \   | /
    //      | |      | |      | |      \ |/     | |      | |      \ |/
    //      | |      | v      | V      | |      | |      | V      | |
    //      | |     [   ]    [   ]     | |      | |     [   ]     | |
    //      | |     [   ]    [   ]     | |      | |     [   ]     | |
    //      | |      ^ |      ^ |      | |      | |      ^ |      | |
    //     /|  \     | |      | |      | |      | |      | |      | |
    //    / |   \    | |      | |      | V      | |      | |      | |
    //   | [   ] |   | |      | |     [   ]     | |      | |      | |
    //   | [   ] |   | |      | |     [   ]     | |      | |      | |
    //    \   | /    | |      | |      ^ |      | |      | |      | |
    //     \  |/    /|  \     | |      | |      | |      | |      | |
    //      | |    / |   \    | |      | |      | |      | |      | v
    //      | |   | [   ] |   | |      | |      | |      | |     [   ]
    //      | |   | [   ] |   | |      | |      | |      | |     [   ]
    //      | |    \   | /    | |      | |      | |      | |      ^ |
    //      | |     \  |/     | |      | |     /|  \     | |     /|  \
    //      | |      | |      | |      | V    / |   \    | |    / |   \
    //      | |      | |      | |     [   ]  | [   ] |   | |   | [   ] |
    //      | |      | |      | |     [   ]  | [   ] |   | |   | [   ] |
    //      | |      | |      | |      ^ |    \   | /    | |    \   | /
    //      | |      | |      | |      | |     \  |/     | |     \  |/

    // Arrage - Setup the linked table
    let mut linked_table = LinkedTable::default();
    // Column headers
    linked_table.table[0][0] = Link::ColumnHeader(ColumnHeader {
        cell_count: 2,
        up: Some(4),
        down: Some(2),
        left: Some(6),
        right: Some(1),
    });
    linked_table.table[0][1] = Link::ColumnHeader(ColumnHeader {
        cell_count: 2,
        up: Some(5),
        down: Some(3),
        left: Some(0),
        right: Some(2),
    });
    linked_table.table[0][2] = Link::ColumnHeader(ColumnHeader {
        cell_count: 2,
        up: Some(3),
        down: Some(1),
        left: Some(1),
        right: Some(3),
    });
    linked_table.table[0][3] = Link::ColumnHeader(ColumnHeader {
        cell_count: 3,
        up: Some(6),
        down: Some(2),
        left: Some(2),
        right: Some(4),
    });
    linked_table.table[0][4] = Link::ColumnHeader(ColumnHeader {
        cell_count: 2,
        up: Some(6),
        down: Some(1),
        left: Some(3),
        right: Some(5),
    });
    linked_table.table[0][5] = Link::ColumnHeader(ColumnHeader {
        cell_count: 2,
        up: Some(3),
        down: Some(1),
        left: Some(4),
        right: Some(6),
    });
    linked_table.table[0][6] = Link::ColumnHeader(ColumnHeader {
        cell_count: 3,
        up: Some(6),
        down: Some(2),
        left: Some(5),
        right: Some(0),
    });
    // Row 1
    linked_table.table[1][2] = Link::Cell(Cell {
        column_index: 2,
        row_index: 1,
        up: Some(0),
        down: Some(3),
        left: Some(5),
        right: Some(4),
    });
    linked_table.table[1][4] = Link::Cell(Cell {
        column_index: 4,
        row_index: 1,
        up: Some(0),
        down: Some(6),
        left: Some(2),
        right: Some(5),
    });
    linked_table.table[1][5] = Link::Cell(Cell {
        column_index: 5,
        row_index: 1,
        up: Some(0),
        down: Some(3),
        left: Some(4),
        right: Some(2),
    });
    // Row 2
    linked_table.table[2][0] = Link::Cell(Cell {
        column_index: 0,
        row_index: 2,
        up: Some(0),
        down: Some(4),
        left: Some(6),
        right: Some(3),
    });
    linked_table.table[2][3] = Link::Cell(Cell {
        column_index: 3,
        row_index: 2,
        up: Some(0),
        down: Some(4),
        left: Some(0),
        right: Some(6),
    });
    linked_table.table[2][6] = Link::Cell(Cell {
        column_index: 6,
        row_index: 2,
        up: Some(0),
        down: Some(5),
        left: Some(3),
        right: Some(0),
    });
    // Row 3
    linked_table.table[3][1] = Link::Cell(Cell {
        column_index: 1,
        row_index: 3,
        up: Some(0),
        down: Some(5),
        left: Some(5),
        right: Some(2),
    });
    linked_table.table[3][2] = Link::Cell(Cell {
        column_index: 2,
        row_index: 3,
        up: Some(1),
        down: Some(0),
        left: Some(1),
        right: Some(5),
    });
    linked_table.table[3][5] = Link::Cell(Cell {
        column_index: 5,
        row_index: 3,
        up: Some(1),
        down: Some(0),
        left: Some(2),
        right: Some(1),
    });
    // Row 4
    linked_table.table[4][0] = Link::Cell(Cell {
        column_index: 0,
        row_index: 4,
        up: Some(2),
        down: Some(0),
        left: Some(3),
        right: Some(3),
    });
    linked_table.table[4][3] = Link::Cell(Cell {
        column_index: 3,
        row_index: 4,
        up: Some(2),
        down: Some(6),
        left: Some(0),
        right: Some(0),
    });
    // Row 5
    linked_table.table[5][1] = Link::Cell(Cell {
        column_index: 1,
        row_index: 5,
        up: Some(3),
        down: Some(0),
        left: Some(6),
        right: Some(6),
    });
    linked_table.table[5][6] = Link::Cell(Cell {
        column_index: 6,
        row_index: 5,
        up: Some(1),
        down: Some(6),
        left: Some(1),
        right: Some(1),
    });
    // Row 6
    linked_table.table[6][3] = Link::Cell(Cell {
        column_index: 3,
        row_index: 6,
        up: Some(4),
        down: Some(0),
        left: Some(6),
        right: Some(4),
    });
    linked_table.table[6][4] = Link::Cell(Cell {
        column_index: 4,
        row_index: 6,
        up: Some(1),
        down: Some(0),
        left: Some(3),
        right: Some(6),
    });
    linked_table.table[6][6] = Link::Cell(Cell {
        column_index: 6,
        row_index: 6,
        up: Some(5),
        down: Some(0),
        left: Some(4),
        right: Some(3),
    });

    // Act - Hide column A (0), then D (3), then G (6)
    let hidden_columns = hide_all_columns_in_row(2, 0, &mut linked_table);

    // Assert - The columns were each hidden in the correct order
    assert_eq!(hidden_columns, vec![0, 3, 6]);
    assert_eq!(
        linked_table.table[0][0],
        Link::ColumnHeader(ColumnHeader {
            cell_count: 2,
            up: Some(4),
            down: Some(2),
            left: Some(6),
            right: Some(1)
        })
    );
    assert_eq!(
        linked_table.table[0][1],
        Link::ColumnHeader(ColumnHeader {
            cell_count: 1,
            up: Some(3),
            down: Some(3),
            left: Some(5),
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
            right: Some(4)
        })
    );
    assert_eq!(
        linked_table.table[0][3],
        Link::ColumnHeader(ColumnHeader {
            cell_count: 1,
            up: Some(6),
            down: Some(6),
            left: Some(2),
            right: Some(4)
        })
    );
    assert_eq!(
        linked_table.table[0][4],
        Link::ColumnHeader(ColumnHeader {
            cell_count: 1,
            up: Some(1),
            down: Some(1),
            left: Some(2),
            right: Some(5)
        })
    );
    assert_eq!(
        linked_table.table[0][5],
        Link::ColumnHeader(ColumnHeader {
            cell_count: 2,
            up: Some(3),
            down: Some(1),
            left: Some(4),
            right: Some(1)
        })
    );
    assert_eq!(
        linked_table.table[0][6],
        Link::ColumnHeader(ColumnHeader {
            cell_count: 1,
            up: Some(5),
            down: Some(5),
            left: Some(5),
            right: Some(1)
        })
    );
    // Row 1
    assert_eq!(
        linked_table.table[1][2],
        Link::Cell(Cell {
            column_index: 2,
            row_index: 1,
            up: Some(0),
            down: Some(3),
            left: Some(5),
            right: Some(4)
        })
    );
    assert_eq!(
        linked_table.table[1][4],
        Link::Cell(Cell {
            column_index: 4,
            row_index: 1,
            up: Some(0),
            down: Some(0),
            left: Some(2),
            right: Some(5)
        })
    );
    assert_eq!(
        linked_table.table[1][5],
        Link::Cell(Cell {
            column_index: 5,
            row_index: 1,
            up: Some(0),
            down: Some(3),
            left: Some(4),
            right: Some(2)
        })
    );
    // Row 2
    assert_eq!(
        linked_table.table[2][0],
        Link::Cell(Cell {
            column_index: 0,
            row_index: 2,
            up: Some(0),
            down: Some(4),
            left: Some(6),
            right: Some(3)
        })
    );
    assert_eq!(
        linked_table.table[2][3],
        Link::Cell(Cell {
            column_index: 3,
            row_index: 2,
            up: Some(0),
            down: Some(4),
            left: Some(0),
            right: Some(6)
        })
    );
    assert_eq!(
        linked_table.table[2][6],
        Link::Cell(Cell {
            column_index: 6,
            row_index: 2,
            up: Some(0),
            down: Some(5),
            left: Some(3),
            right: Some(0)
        })
    );
    // Row 3
    assert_eq!(
        linked_table.table[3][1],
        Link::Cell(Cell {
            column_index: 1,
            row_index: 3,
            up: Some(0),
            down: Some(0),
            left: Some(5),
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
            right: Some(5)
        })
    );
    assert_eq!(
        linked_table.table[3][5],
        Link::Cell(Cell {
            column_index: 5,
            row_index: 3,
            up: Some(1),
            down: Some(0),
            left: Some(2),
            right: Some(1)
        })
    );
    // Row 4
    assert_eq!(
        linked_table.table[4][0],
        Link::Cell(Cell {
            column_index: 0,
            row_index: 4,
            up: Some(2),
            down: Some(0),
            left: Some(3),
            right: Some(3)
        })
    );
    assert_eq!(
        linked_table.table[4][3],
        Link::Cell(Cell {
            column_index: 3,
            row_index: 4,
            up: Some(0),
            down: Some(6),
            left: Some(0),
            right: Some(0),
        })
    );
    // Row 5
    assert_eq!(
        linked_table.table[5][1],
        Link::Cell(Cell {
            column_index: 1,
            row_index: 5,
            up: Some(3),
            down: Some(0),
            left: Some(6),
            right: Some(6)
        })
    );
    assert_eq!(
        linked_table.table[5][6],
        Link::Cell(Cell {
            column_index: 6,
            row_index: 5,
            up: Some(0),
            down: Some(0),
            left: Some(1),
            right: Some(1)
        })
    );
    // Row 6
    assert_eq!(
        linked_table.table[6][3],
        Link::Cell(Cell {
            column_index: 3,
            row_index: 6,
            up: Some(0),
            down: Some(0),
            left: Some(6),
            right: Some(4)
        })
    );
    assert_eq!(
        linked_table.table[6][4],
        Link::Cell(Cell {
            column_index: 4,
            row_index: 6,
            up: Some(1),
            down: Some(0),
            left: Some(3),
            right: Some(6)
        })
    );
    assert_eq!(
        linked_table.table[6][6],
        Link::Cell(Cell {
            column_index: 6,
            row_index: 6,
            up: Some(5),
            down: Some(0),
            left: Some(4),
            right: Some(3)
        })
    );
}

#[test]
fn it_reveals_all_columns_for_a_row() {
    let original_table = generate_crafted_hide_all_columns_in_row_table();
    let mut linked_table = generate_crafted_hide_all_columns_in_row_table();

    hide_all_columns_in_row(2, 0, &mut linked_table);
    reveal_all_columns_in_row(2, 0, &mut linked_table);

    assert_crafted_table_eq(&linked_table, &original_table);
}

#[test]
fn it_reveals_multiple_hidden_rows_in_reverse_order() {
    let original_table = generate_crafted_hide_all_columns_in_row_table();
    let mut linked_table = generate_crafted_hide_all_columns_in_row_table();

    hide_all_columns_in_row(4, 0, &mut linked_table);
    hide_all_columns_in_row(5, 1, &mut linked_table);
    hide_all_columns_in_row(1, 2, &mut linked_table);

    reveal_all_columns_in_row(1, 2, &mut linked_table);
    reveal_all_columns_in_row(5, 1, &mut linked_table);
    reveal_all_columns_in_row(4, 0, &mut linked_table);

    assert_crafted_table_eq(&linked_table, &original_table);
}

#[test]
fn launch_dancing_links_returns_an_exact_cover_solution() {
    let decisions = launch_dancing_links(
        1,
        DecisionStrategy::First,
        DecisionStrategy::First,
    )[0];
    let constraint_table = generate_constraint_table();
    let unique_decisions: HashSet<_> = decisions.iter().copied().collect();

    assert_eq!(decisions.len(), 81);
    assert_eq!(unique_decisions.len(), 81);

    for column_idx in 0..LINKED_TABLE_COLUMNS {
        let cells_in_column = decisions
            .iter()
            .filter(|&&row_idx| constraint_table.table[row_idx][column_idx])
            .count();

        assert_eq!(
            cells_in_column, 1,
            "column {column_idx} was not covered exactly once"
        );
    }
}

#[test]
fn linked_row_indexes_are_offset_by_column_header_row() {
    let linked_table = generate_linked_table();

    for linked_row_idx in 1..LINKED_TABLE_ROWS {
        let row_index = linked_table.table[linked_row_idx]
            .iter()
            .find_map(|link| match link {
                Link::Cell(cell) => Some(cell.row_index),
                _ => None,
            })
            .unwrap();

        assert_eq!(row_index, linked_row_idx - 1);
    }
}

#[test]
fn solution_rows_map_to_expected_board_cells() {
    let board = build_partial_board(&[1, 11, 729]);

    assert_eq!(board.get(0, 0), 1);
    assert_eq!(board.get(1, 0), 2);
    assert_eq!(board.get(8, 8), 9);
}

#[test]
fn preview_board_mapping_matches_solver_solution_rows() {
    let exact_cover_solution = launch_dancing_links(
        1,
        DecisionStrategy::First,
        DecisionStrategy::First,
    )[0];
    let linked_solution_rows: Vec<usize> =
        exact_cover_solution.iter().map(|row| row + 1).collect();

    let preview_board = build_partial_board(&linked_solution_rows);

    assert_board_is_solved_and_valid(&preview_board);
}

#[test]
fn it_renders_visualization_frames() {
    let output_dir = std::env::temp_dir().join(format!(
        "dancing_links_visualization_test_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&output_dir);

    let result = visualize_dancing_links_search(
        1,
        DecisionStrategy::First,
        DecisionStrategy::First,
        DancingLinksVisualizationConfig {
            output_dir: output_dir.clone(),
            max_frames: Some(3),
            ..DancingLinksVisualizationConfig::default()
        },
    )
    .unwrap();

    assert_eq!(result.solutions.len(), 1);
    assert_eq!(result.frames.len(), 3);
    for frame in &result.frames {
        assert!(frame.exists(), "missing frame {}", frame.display());
    }

    fs::remove_dir_all(output_dir).unwrap();
}

#[test]
fn final_visualization_frame_is_a_valid_solved_board() {
    let output_dir = std::env::temp_dir().join(format!(
        "dancing_links_visualization_solution_test_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&output_dir);

    let result = visualize_dancing_links_search(
        1,
        DecisionStrategy::First,
        DecisionStrategy::First,
        DancingLinksVisualizationConfig {
            output_dir: output_dir.clone(),
            ..DancingLinksVisualizationConfig::default()
        },
    )
    .unwrap();

    assert_eq!(result.solutions.len(), 1);
    let solved_board = &result.solutions[0];
    assert_board_is_solved_and_valid(solved_board);

    let last_frame = result.frames.last().expect("expected at least one frame");
    assert!(
        last_frame
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.contains("solution_1")),
        "last frame should be the solved board, got {}",
        last_frame.display()
    );

    let svg_contents = fs::read_to_string(last_frame).unwrap();
    assert!(
        svg_contents.contains("Step: solution_1"),
        "last svg should be labeled as the solved frame"
    );
    assert!(
        svg_contents.contains("Sudoku Preview"),
        "last svg should include the sudoku preview panel"
    );

    fs::remove_dir_all(output_dir).unwrap();
}

// #[test]
// fn dance_bench() {
//     const RUNS: usize = 10;

//     let mut total = Duration::ZERO;

//     for i in 0..RUNS {
//         let start = Instant::now();
//         let _solutions = advanced_get_sudoku_boards(
//             100000,
//             DecisionStrategy::Optimal,
//             DecisionStrategy::Optimal,
//         );
//         let elapsed = start.elapsed();

//         println!("Run {} took {:?}", i + 1, elapsed);
//         total += elapsed;
//     }

//     let average = total / RUNS as u32;
//     println!("\nAverage runtime over {} runs: {:?}", RUNS, average);

//     assert!(false);
// }
